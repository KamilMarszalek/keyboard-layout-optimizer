use super::common::{ASCII_COUNT, AsciiChar, KeyIndex};
use super::modifier::Modifier;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct KeySymbol {
    pub base: AsciiChar,
    pub shifted: AsciiChar,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Layout<const N: usize> {
    mappings: [KeySymbol; N],
    symbol_to_key: [Option<KeyIndex>; ASCII_COUNT],
}

impl<const N: usize> Layout<N> {
    /// Builds a keyboard layout from base symbols and a modifier mapping.
    ///
    /// The provided `symbols` must match `modifier.base_symbols()`.
    /// For each key, the layout stores both the base symbol and the symbol
    /// produced by applying the modifier. It also builds a reverse lookup for
    /// both forms.
    pub fn new(symbols: &[AsciiChar; N], modifier: &Modifier) -> Result<Self, String> {
        if !Self::is_permutation(symbols, modifier.base_symbols()) {
            return Err("Provided symbols do not match modifier's base symbols".to_string());
        }

        let mappings: [KeySymbol; N] = std::array::from_fn(|i| KeySymbol {
            base: symbols[i],
            shifted: modifier.shift(symbols[i]).unwrap(),
        });

        let mut symbol_to_key = [None; ASCII_COUNT];
        for (key_idx, symbol) in mappings.iter().enumerate() {
            symbol_to_key[symbol.base as usize] = Some(key_idx);
            symbol_to_key[symbol.shifted as usize] = Some(key_idx);
        }

        Ok(Self { mappings, symbol_to_key })
    }

    /// Swaps two key positions in the layout.
    ///
    /// This updates both the forward key mappings and the reverse symbol lookup.
    /// Passing the same index twice is a no-op.
    pub fn swap(&mut self, first: KeyIndex, second: KeyIndex) {
        if first == second {
            return;
        }
        let first_symbol = self.mappings[first];
        let second_symbol = self.mappings[second];

        self.mappings.swap(first, second);

        self.symbol_to_key[first_symbol.base as usize] = Some(second);
        self.symbol_to_key[first_symbol.shifted as usize] = Some(second);

        self.symbol_to_key[second_symbol.base as usize] = Some(first);
        self.symbol_to_key[second_symbol.shifted as usize] = Some(first);
    }

    pub fn key_of(&self, symbol: AsciiChar) -> Option<KeyIndex> {
        self.symbol_to_key[symbol as usize]
    }

    pub fn mappings_iter(&self) -> impl Iterator<Item = &KeySymbol> {
        self.mappings.iter()
    }

    fn is_permutation(symbols: &[AsciiChar], alphabet: &[AsciiChar]) -> bool {
        let mut counter = [0i32; ASCII_COUNT];

        for &symbol in symbols {
            counter[symbol as usize] += 1;
        }

        for &symbol in alphabet {
            counter[symbol as usize] -= 1;
        }

        counter.iter().all(|c| *c == 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::preset::constants::US_KEY_COUNT;

    use super::*;
    use rstest::rstest;

    fn test_modifier() -> Modifier {
        Modifier::new([(b'a', b'A'), (b'b', b'B')]).unwrap()
    }

    fn test_layout() -> Layout<2> {
        Layout::new(b"ab", &test_modifier()).unwrap()
    }

    fn assert_layout_new_rejects_symbols<const N: usize>(symbols: &[AsciiChar; N]) {
        let modifier = test_modifier();
        let layout = Layout::new(symbols, &modifier);

        assert_eq!(layout.err().unwrap(), "Provided symbols do not match modifier's base symbols");
    }

    #[test]
    fn layout_new_rejects_non_matching_symbols() {
        assert_layout_new_rejects_symbols(&[]);
        assert_layout_new_rejects_symbols(b"a");
        assert_layout_new_rejects_symbols(b"abc");
    }

    #[test]
    fn layout_new_succeeds() {
        let modifier = test_modifier();
        let layout = Layout::new(b"ab", &modifier);
        assert!(layout.is_ok())
    }

    #[test]
    fn layout_new_mappings() {
        let modifier = test_modifier();
        let layout = Layout::new(b"ab", &modifier).unwrap();
        for key_symbol in layout.mappings {
            assert!(modifier.base_symbols().contains(&key_symbol.base));
            assert_eq!(key_symbol.shifted, modifier.shift(key_symbol.base).unwrap());
        }
    }

    #[test]
    fn layout_new_symbol_to_key() {
        let modifier = test_modifier();
        let symbols = [b'a', b'b'];
        let layout = Layout::new(&symbols, &modifier).unwrap();

        for (i, symbol) in symbols.iter().enumerate() {
            assert_eq!(layout.key_of(*symbol), Some(i));
            assert_eq!(layout.key_of(modifier.shift(*symbol).unwrap()), Some(i));
        }
    }

    #[rstest]
    #[case::first_key(0, KeySymbol { base: b'b', shifted: b'B' })]
    #[case::second_key(1, KeySymbol { base: b'a', shifted: b'A' })]
    fn layout_swap_updates_mappings(#[case] key: KeyIndex, #[case] expected: KeySymbol) {
        let mut layout = test_layout();

        layout.swap(0, 1);

        assert_eq!(layout.mappings[key], expected);
    }

    #[rstest]
    #[case::base_a(b'a', Some(1))]
    #[case::shifted_a(b'A', Some(1))]
    #[case::base_b(b'b', Some(0))]
    #[case::shifted_b(b'B', Some(0))]
    fn layout_swap_updates_symbol_to_key(
        #[case] symbol: AsciiChar,
        #[case] expected_key: Option<KeyIndex>,
    ) {
        let mut layout = test_layout();

        layout.swap(0, 1);

        assert_eq!(layout.key_of(symbol), expected_key);
    }

    fn assert_original_symbol_to_key(layout: &Layout<2>) {
        assert_eq!(layout.key_of(b'a'), Some(0));
        assert_eq!(layout.key_of(b'A'), Some(0));
        assert_eq!(layout.key_of(b'b'), Some(1));
        assert_eq!(layout.key_of(b'B'), Some(1));
    }

    #[test]
    fn layout_swap_same_index_is_noop() {
        let mut layout = test_layout();
        let before = layout.mappings;

        layout.swap(0, 0);

        assert_eq!(layout.mappings, before);
        assert_original_symbol_to_key(&layout);
    }

    #[test]
    fn layout_swap_twice_restores_original_layout() {
        let mut layout = test_layout();
        let before = layout.mappings;

        layout.swap(0, 1);
        layout.swap(0, 1);

        assert_eq!(layout.mappings, before);
        assert_original_symbol_to_key(&layout);
    }

    #[test]
    fn layout_qwerty_us_has_printable_key_count_mappings() {
        let modifier = Modifier::standard_us();
        let layout = Layout::qwerty_us(&modifier);
        assert_eq!(layout.mappings.len(), US_KEY_COUNT);
    }

    #[test]
    fn layout_qwerty_us_uses_modifier_alphabet() {
        let modifier = Modifier::standard_us();
        let layout = Layout::qwerty_us(&modifier);

        let mut layout_symbols: Vec<AsciiChar> =
            layout.mappings.iter().map(|mapping| mapping.base).collect();
        let mut modifier_symbols = modifier.base_symbols().to_vec();
        layout_symbols.sort();
        modifier_symbols.sort();

        assert_eq!(layout_symbols, modifier_symbols);
    }

    #[test]
    fn layout_dvorak_us_uses_modifier_alphabet() {
        let modifier = Modifier::standard_us();
        let layout = Layout::dvorak_us(&modifier);

        let mut layout_symbols: Vec<AsciiChar> =
            layout.mappings.iter().map(|mapping| mapping.base).collect();
        let mut modifier_symbols = modifier.base_symbols().to_vec();
        layout_symbols.sort();
        modifier_symbols.sort();

        assert_eq!(layout_symbols, modifier_symbols);
    }

    #[rstest]
    #[case::qwerty_top_q(Layout::qwerty_us(&Modifier::standard_us()), b'q', 13)]
    #[case::qwerty_home_a(Layout::qwerty_us(&Modifier::standard_us()), b'a', 26)]
    #[case::dvorak_home_a(Layout::dvorak_us(&Modifier::standard_us()), b'a', 26)]
    #[case::dvorak_home_o(Layout::dvorak_us(&Modifier::standard_us()), b'o', 27)]
    #[case::dvorak_home_s(Layout::dvorak_us(&Modifier::standard_us()), b's', 35)]
    fn preset_layout_places_symbols_at_expected_keys(
        #[case] layout: Layout<{ US_KEY_COUNT }>,
        #[case] symbol: AsciiChar,
        #[case] expected_key: KeyIndex,
    ) {
        assert_eq!(layout.key_of(symbol), Some(expected_key));
    }
}
