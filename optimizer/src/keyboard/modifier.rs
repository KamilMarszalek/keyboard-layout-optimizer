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
