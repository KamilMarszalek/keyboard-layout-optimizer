use super::common::{AsciiChar, KEY_COUNT};
use super::modifier::{Modifier, StandardUSModifier};
use std::collections::HashMap;

#[derive(Clone, Copy)]
struct KeySymbol {
    base: AsciiChar,
    shifted: AsciiChar,
}

pub struct Layout {
    mappings: [KeySymbol; KEY_COUNT],
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

        Ok(Self { mappings })
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
}
