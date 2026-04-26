use crate::keyboard::common::{ASCII_COUNT, KeyIndex};

use super::common::{AsciiChar, KEY_COUNT};
use super::modifier::{Modifier, StandardUSModifier};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct KeySymbol {
    pub base: AsciiChar,
    pub shifted: AsciiChar,
}

#[derive(Clone)]
pub struct Layout {
    pub mappings: [KeySymbol; KEY_COUNT],
    symbol_to_key: [Option<KeyIndex>; ASCII_COUNT] 
}

impl Layout {
    pub fn new<T: Modifier>(symbols: &[AsciiChar; KEY_COUNT], modifier: &T) -> Result<Self, String> {
        if !Self::is_permutation(symbols, modifier.base_symbols()) {
            return Err("Provided symbols do not match modifier's alphabet".to_string());
        }

        let mut mappings = [KeySymbol { base: symbols[0], shifted: symbols[0] }; KEY_COUNT];

        for (i, &base) in symbols.iter().enumerate() {
            let shifted = modifier.shift(base).map_err(|e| e.to_string())?;
            mappings[i].base = base;
            mappings[i].shifted = shifted;
        }

        Ok(Self::from_mappings(mappings))
    }

    fn from_mappings(mappings: [KeySymbol; KEY_COUNT]) -> Self {
        let mut symbol_to_key = [None; ASCII_COUNT];
        for (key_idx, symbol) in mappings.iter().enumerate() {
            symbol_to_key[symbol.base as usize] = Some(key_idx);
            symbol_to_key[symbol.shifted as usize] = Some(key_idx);
        }
        Self {mappings, symbol_to_key}
    }

    pub fn standard_us() -> Self {
        let modifier = StandardUSModifier::new();
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
        let mut symbols_map = HashMap::new();
        let mut alphabet_map = HashMap::new();

        for &symbol in symbols {
            *symbols_map.entry(symbol).or_insert(0) += 1;
        }

        for &symbol in alphabet {
            *alphabet_map.entry(symbol).or_insert(0) += 1;
        }

        symbols_map == alphabet_map
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
