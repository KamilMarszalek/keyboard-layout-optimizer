use super::common::{ASCII_COUNT, AsciiChar, KEY_COUNT, KeyIndex};
use super::modifier::Modifier;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct KeySymbol {
    pub base: AsciiChar,
    pub shifted: AsciiChar,
}

#[derive(Clone)]
pub struct Layout<const N: usize> {
    pub mappings: [KeySymbol; N],
    symbol_to_key: [Option<KeyIndex>; ASCII_COUNT],
}

impl<const N: usize> Layout<N> {
    pub fn new(symbols: &[AsciiChar; N], modifier: &Modifier) -> Result<Self, String> {
        if !Self::is_permutation(symbols, modifier.base_symbols()) {
            return Err("Provided symbols do not match modifier's base symbols".to_string());
        }

        let mappings: [KeySymbol; N] =
            std::array::from_fn(|i| KeySymbol { base: symbols[i], shifted: modifier.shift(symbols[i]).unwrap() });

        let mut symbol_to_key = [None; ASCII_COUNT];
        for (key_idx, symbol) in mappings.iter().enumerate() {
            symbol_to_key[symbol.base as usize] = Some(key_idx);
            symbol_to_key[symbol.shifted as usize] = Some(key_idx);
        }

        Ok(Self { mappings, symbol_to_key })
    }

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

impl Layout<KEY_COUNT> {
    pub fn standard_us() -> Self {
        let modifier = Modifier::standard_us();
        #[rustfmt::skip]
        let symbols: [AsciiChar; KEY_COUNT] = [
            b'`', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=',
            b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\\',
            b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'',
            b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/',
        ];
        Self::new(&symbols, &modifier).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_modifier() -> Modifier {
        Modifier::new([(b'a', b'A'), (b'b', b'B')])
    }

    fn test_layout() -> Layout<2> {
        Layout::new(b"ab", &test_modifier()).unwrap()
    }

    #[test]
    fn layout_new_fails_when_empty_symbols() {
        let modifer = test_modifier();
        let layout = Layout::new(&[], &modifer);
        assert_eq!(layout.err().unwrap(), "Provided symbols do not match modifier's base symbols");
    }

    #[test]
    fn layout_new_fails_when_incomplete_symbols() {
        let modifer = test_modifier();
        let layout = Layout::new(b"a", &modifer);
        assert_eq!(layout.err().unwrap(), "Provided symbols do not match modifier's base symbols");
    }

    #[test]
    fn layout_new_fails_when_too_many_symbols() {
        let modifer = test_modifier();
        let layout = Layout::new(b"abc", &modifer);
        assert_eq!(layout.err().unwrap(), "Provided symbols do not match modifier's base symbols");
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

    #[test]
    fn layout_swap_updates_mappings() {
        let mut layout = test_layout();

        layout.swap(0, 1);

        assert_eq!(layout.mappings[0], KeySymbol { base: b'b', shifted: b'B' });
        assert_eq!(layout.mappings[1], KeySymbol { base: b'a', shifted: b'A' });
    }

    #[test]
    fn layout_swap_updates_symbol_to_key() {
        let mut layout = test_layout();

        layout.swap(0, 1);

        assert_eq!(layout.key_of(b'a'), Some(1));
        assert_eq!(layout.key_of(b'A'), Some(1));
        assert_eq!(layout.key_of(b'b'), Some(0));
        assert_eq!(layout.key_of(b'B'), Some(0));
    }

    #[test]
    fn layout_swap_same_index_is_noop() {
        let mut layout = test_layout();
        let before = layout.mappings;

        layout.swap(0, 0);

        assert_eq!(layout.mappings, before);
        assert_eq!(layout.key_of(b'a'), Some(0));
        assert_eq!(layout.key_of(b'A'), Some(0));
        assert_eq!(layout.key_of(b'b'), Some(1));
        assert_eq!(layout.key_of(b'B'), Some(1));
    }

    #[test]
    fn layout_swap_twice_restores_original_layout() {
        let mut layout = test_layout();
        let before = layout.mappings;

        layout.swap(0, 1);
        layout.swap(0, 1);

        assert_eq!(layout.mappings, before);
        assert_eq!(layout.key_of(b'a'), Some(0));
        assert_eq!(layout.key_of(b'A'), Some(0));
        assert_eq!(layout.key_of(b'b'), Some(1));
        assert_eq!(layout.key_of(b'B'), Some(1));
    }

    #[test]
    fn layout_standard_us_has_key_count_mappings() {
        let layout = Layout::standard_us();
        assert_eq!(layout.mappings.len(), KEY_COUNT);
    }

    #[test]
    fn layout_standard_us_uses_modifier_alphabet() {
        let layout = Layout::standard_us();
        let modifier = Modifier::standard_us();

        let mut layout_symbols: Vec<AsciiChar> = layout.mappings.iter().map(|mapping| mapping.base).collect();
        let mut modifier_symbols = modifier.base_symbols().to_vec();
        layout_symbols.sort();
        modifier_symbols.sort();

        assert_eq!(layout_symbols, modifier_symbols);
    }
}
