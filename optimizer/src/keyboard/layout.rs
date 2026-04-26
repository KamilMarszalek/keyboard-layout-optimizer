use super::common::{ASCII_COUNT, AsciiChar, KEY_COUNT, KeyIndex};
use super::modifier::Modifier;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct KeySymbol {
    pub base: AsciiChar,
    pub shifted: AsciiChar,
}

#[derive(Clone)]
pub struct Layout {
    pub mappings: [KeySymbol; KEY_COUNT],
    symbol_to_key: [Option<KeyIndex>; ASCII_COUNT],
}

impl Layout {
    pub fn new(symbols: &[AsciiChar; KEY_COUNT], modifier: &Modifier) -> Result<Self, String> {
        if !Self::is_permutation(symbols, modifier.base_symbols()) {
            return Err("Provided symbols do not match modifier's alphabet".to_string());
        }

        let mut mappings = [KeySymbol { base: symbols[0], shifted: symbols[0] }; KEY_COUNT];
        for (slot, &base) in mappings.iter_mut().zip(symbols.iter()) {
            let shifted = modifier.shift(base).map_err(|e| e.to_string())?;
            *slot = KeySymbol { base, shifted }
        }

        Ok(Self::from_mappings(mappings))
    }

    fn from_mappings(mappings: [KeySymbol; KEY_COUNT]) -> Self {
        let mut symbol_to_key = [None; ASCII_COUNT];
        for (key_idx, symbol) in mappings.iter().enumerate() {
            symbol_to_key[symbol.base as usize] = Some(key_idx);
            symbol_to_key[symbol.shifted as usize] = Some(key_idx);
        }
        Self { mappings, symbol_to_key }
    }

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

    pub fn key_of(&self, symbol: KeyIndex) -> Option<KeyIndex> {
        self.symbol_to_key[symbol]
    }
}
