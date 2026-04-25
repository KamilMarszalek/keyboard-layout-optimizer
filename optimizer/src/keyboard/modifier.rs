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
                write!(f, "Symbol {} cannot is not considered as 'base' for this modifier", c)
            }
        }
    }
}

pub trait Modifier {
    fn shift(&self, c: AsciiChar) -> Result<AsciiChar, ModifierError>;
    fn base_symbols(&self) -> &[AsciiChar];
}

pub struct StandardUSModifier {
    encode: HashMap<AsciiChar, AsciiChar>,
    symbols: Vec<AsciiChar>,
}

impl Modifier for StandardUSModifier {
    fn shift(&self, c: AsciiChar) -> Result<AsciiChar, ModifierError> {
        self.encode.get(&c).copied().ok_or(ModifierError::UnsupportedBase(c))
    }

    fn base_symbols(&self) -> &[AsciiChar] {
        &self.symbols
    }
}

impl StandardUSModifier {
    pub fn new() -> Self {
        let mut encode = HashMap::new();

        for c in b'a'..=b'z' {
            let shifted = c.to_ascii_uppercase();
            encode.insert(c, shifted);
        }

        let symbols = [
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

        for &(a, b) in &symbols {
            encode.insert(a, b);
        }

        let symbols = encode.keys().copied().collect();

        Self { encode: encode, symbols: symbols }
    }
}
