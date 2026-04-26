use super::common::AsciiChar;
use core::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ModifierError {
    UnsupportedBase(AsciiChar),
}

impl fmt::Display for ModifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModifierError::UnsupportedBase(c) => {
                write!(f, "Symbol {} cannot is not considered as 'base' for this modifier", *c as char)
            }
        }
    }
}

pub struct Modifier {
    encode: HashMap<AsciiChar, AsciiChar>,
    symbols: Vec<AsciiChar>,
}

impl Modifier {
    pub fn new<I>(shift_pairs: I) -> Self
    where
        I: IntoIterator<Item = (AsciiChar, AsciiChar)>,
    {
        let mut encode = HashMap::new();
        for (a, b) in shift_pairs {
            encode.insert(a, b);
        }

        let symbols = encode.keys().copied().collect();
        Self { encode, symbols }
    }

    pub fn shift(&self, c: AsciiChar) -> Result<AsciiChar, ModifierError> {
        self.encode.get(&c).copied().ok_or(ModifierError::UnsupportedBase(c))
    }

    pub fn base_symbols(&self) -> &[AsciiChar] {
        &self.symbols
    }

    pub fn standard_us() -> Self {
        let letter_pairs = (b'a'..=b'z').map(|c| (c, c.to_ascii_uppercase()));

        let punctuation_pairs = [
            (b'1', b'!'),
            (b'2', b'@'),
            (b'3', b'#'),
            (b'4', b'$'),
            (b'5', b'%'),
            (b'6', b'^'),
            (b'7', b'&'),
            (b'8', b'*'),
            (b'9', b'('),
            (b'0', b')'),
            (b'-', b'_'),
            (b'=', b'+'),
            (b'[', b'{'),
            (b']', b'}'),
            (b'\\', b'|'),
            (b';', b':'),
            (b'\'', b'"'),
            (b',', b'<'),
            (b'.', b'>'),
            (b'/', b'?'),
            (b'`', b'~'),
        ];

        Self::new(letter_pairs.chain(punctuation_pairs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyboard::{common::KEY_COUNT, modifier};

    #[test]
    fn shift_supported() {
        let modifier = Modifier::new([(b'a', b'A'), (b'z', b'Z'), (b'1', b'!'), (b'/', b'?')]);
        assert_eq!(modifier.shift(b'a').unwrap(), b'A');
        assert_eq!(modifier.shift(b'z').unwrap(), b'Z');
        assert_eq!(modifier.shift(b'1').unwrap(), b'!');
        assert_eq!(modifier.shift(b'/').unwrap(), b'?');
    }

    #[test]
    fn shift_unsupported() {
        let modifier = Modifier::new([(b'a', b'A'), (b'1', b'!')]);
        assert!(matches!(modifier.shift(b'B'), Err(ModifierError::UnsupportedBase(b'B'))));
        assert!(matches!(modifier.shift(b'/'), Err(ModifierError::UnsupportedBase(b'/'))));
    }

    #[test]
    fn base_symbols() {
        let modifier = Modifier::new([(b'a', b'A'), (b'1', b'!')]);
        let mut symbols: Vec<AsciiChar> = modifier.base_symbols().to_vec();
        symbols.sort();
        assert_eq!(symbols, [b'1', b'a']);
    }

    #[test]
    fn standard_us() {
        let modifier = Modifier::standard_us();
        assert_eq!(modifier.base_symbols().len(), KEY_COUNT);
        for &symbol in modifier.base_symbols().iter() {
            modifier.shift(symbol).unwrap();
        }
    }
}
