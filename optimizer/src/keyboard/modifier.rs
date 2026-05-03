use crate::keyboard::model::KeyPress;

use super::common::AsciiChar;
use core::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum ModifierError {
    UnsupportedBase(AsciiChar),
    DuplicateBase(AsciiChar),
    DuplicateShifted(AsciiChar),
    AmbiguousSymbol(AsciiChar),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedPressesError {
    InvalidSupportedPressCount { expected: usize, actual: usize },
    MissingBaseKeyPress { base: u8 },
    MissingShiftMapping { base: u8 },
    MissingShiftedKeyPress { base: u8, shifted: u8 },
}
impl fmt::Display for ModifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModifierError::UnsupportedBase(c) => {
                write!(f, "Symbol {} is not a base symbol for this modifier", *c as char)
            }
            ModifierError::DuplicateBase(c) => {
                write!(f, "Symbol {} is already used as a base symbol", *c as char)
            }
            ModifierError::DuplicateShifted(c) => {
                write!(f, "Symbol {} is already used as a shifted symbol", *c as char)
            }
            ModifierError::AmbiguousSymbol(c) => {
                write!(f, "Symbol {} cannot be used as both a base and shifted symbol", *c as char)
            }
        }
    }
}

/// Maps base symbols to shifted symbols and input symbols to logical key presses.
///
/// A `Modifier` defines the printable symbol produced when a modifier such as Shift
/// is applied to a base key symbol (`a` -> `A`, `1` -> `!`). It also supports
/// reverse lookup from an input symbol to a [`KeyPress`], preserving whether Shift
/// is required.
pub struct Modifier {
    encode: HashMap<AsciiChar, AsciiChar>,
    decode: HashMap<AsciiChar, KeyPress>,
    symbols: Vec<AsciiChar>,
}

impl Modifier {
    /// Builds a modifier from `(base, shifted)` symbol pairs.
    pub fn new<I>(shift_pairs: I) -> Result<Self, ModifierError>
    where
        I: IntoIterator<Item = (AsciiChar, AsciiChar)>,
    {
        let mut symbols = Vec::new();
        let mut encode = HashMap::new();
        let mut decode = HashMap::new();
        for (base, shift) in shift_pairs {
            if encode.contains_key(&base) {
                return Err(ModifierError::DuplicateBase(base));
            }
            if decode.contains_key(&shift) {
                return Err(ModifierError::DuplicateShifted(shift));
            }
            if base == shift {
                return Err(ModifierError::AmbiguousSymbol(base));
            }
            if decode.contains_key(&base) {
                return Err(ModifierError::AmbiguousSymbol(base));
            }
            if encode.contains_key(&shift) {
                return Err(ModifierError::AmbiguousSymbol(shift));
            }

            symbols.push(base);
            encode.insert(base, shift);
            decode.insert(base, KeyPress { base, shifted: false });
            decode.insert(shift, KeyPress { base, shifted: true });
        }

        Ok(Self { encode, decode, symbols })
    }

    pub fn shift(&self, c: AsciiChar) -> Result<AsciiChar, ModifierError> {
        self.encode.get(&c).copied().ok_or(ModifierError::UnsupportedBase(c))
    }

    /// Returns the base symbols supported by this modifier.
    /// These symbols define the alphabet that a compatible `Layout` must contain.
    pub fn base_symbols(&self) -> &[AsciiChar] {
        &self.symbols
    }

    /// Returns the standard US Shift mapping for the main printable keyboard symbols.
    /// This includes lowercase Latin letters, digits, and punctuation used by the
    /// main alphanumeric section of a US keyboard layout.
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
            .expect("standard US modifier mapping is valid")
    }

    /// Converts an input symbol to a logical key press.
    ///
    /// Base symbols are returned with `shifted=false`, while shifted symbols are
    /// mapped back to their base symbol with `shifted` set to true
    pub fn key_press_of(&self, symbol: AsciiChar) -> Option<KeyPress> {
        self.decode.get(&symbol).copied()
    }

    pub fn supported_presses<const P: usize>(
        &self,
    ) -> Result<[KeyPress; P], SupportedPressesError> {
        let mut key_presses = Vec::new();

        for &base in self.base_symbols() {
            let base_press = self
                .key_press_of(base)
                .ok_or(SupportedPressesError::MissingBaseKeyPress { base })?;

            key_presses.push(base_press);

            let shifted = self
                .shift(base)
                .map_err(|_| SupportedPressesError::MissingShiftMapping { base })?;

            let shifted_press = self
                .key_press_of(shifted)
                .ok_or(SupportedPressesError::MissingShiftedKeyPress { base, shifted })?;

            key_presses.push(shifted_press);
        }

        key_presses.try_into().map_err(|key_presses: Vec<KeyPress>| {
            SupportedPressesError::InvalidSupportedPressCount {
                expected: P,
                actual: key_presses.len(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyboard::common::KEY_COUNT;

    #[test]
    fn shift_supported() {
        let modifier =
            Modifier::new([(b'a', b'A'), (b'z', b'Z'), (b'1', b'!'), (b'/', b'?')]).unwrap();
        assert_eq!(modifier.shift(b'a').unwrap(), b'A');
        assert_eq!(modifier.shift(b'z').unwrap(), b'Z');
        assert_eq!(modifier.shift(b'1').unwrap(), b'!');
        assert_eq!(modifier.shift(b'/').unwrap(), b'?');
    }

    #[test]
    fn shift_unsupported() {
        let modifier = Modifier::new([(b'a', b'A'), (b'1', b'!')]).unwrap();
        assert!(matches!(modifier.shift(b'B'), Err(ModifierError::UnsupportedBase(b'B'))));
        assert!(matches!(modifier.shift(b'/'), Err(ModifierError::UnsupportedBase(b'/'))));
    }

    #[test]
    fn base_symbols() {
        let modifier = Modifier::new([(b'a', b'A'), (b'1', b'!')]).unwrap();
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

    #[test]
    fn key_press() {
        let modifier = Modifier::standard_us();

        let cases = [
            (b'a', KeyPress { base: b'a', shifted: false }),
            (b'A', KeyPress { base: b'a', shifted: true }),
            (b'1', KeyPress { base: b'1', shifted: false }),
            (b'!', KeyPress { base: b'1', shifted: true }),
            (b'/', KeyPress { base: b'/', shifted: false }),
            (b'?', KeyPress { base: b'/', shifted: true }),
        ];

        for (symbol, expected) in cases {
            assert_eq!(modifier.key_press_of(symbol), Some(expected));
        }
    }

    #[test]
    fn key_press_of_unsupported_symbol_returns_none() {
        let modifier = Modifier::standard_us();

        assert_eq!(modifier.key_press_of(b' '), None);
    }

    #[test]
    fn base_symbols_preserve_input_order() {
        let modifier = Modifier::new([(b'a', b'A'), (b'1', b'!'), (b'/', b'?')]).unwrap();

        assert_eq!(modifier.base_symbols(), [b'a', b'1', b'/']);
    }

    #[test]
    fn modifier_new_returns_error_for_duplicate_base() {
        let result = Modifier::new([(b'a', b'A'), (b'a', b'@')]);

        assert!(matches!(result, Err(ModifierError::DuplicateBase(b'a'))));
    }

    #[test]
    fn modifier_new_returns_error_for_duplicate_shifted_symbol() {
        let result = Modifier::new([(b'a', b'!'), (b'1', b'!')]);

        assert!(matches!(result, Err(ModifierError::DuplicateShifted(b'!'))));
    }

    #[test]
    fn modifier_new_returns_error_when_symbol_is_base_and_shifted() {
        let result = Modifier::new([(b'a', b'A'), (b'A', b'!')]);

        assert!(matches!(result, Err(ModifierError::AmbiguousSymbol(b'A'))));
    }

    #[test]
    fn modifier_new_returns_error_when_base_matches_shifted_symbol() {
        let result = Modifier::new([(b'a', b'a')]);

        assert!(matches!(result, Err(ModifierError::AmbiguousSymbol(b'a'))));
    }

    #[test]
    fn supported_presses_from_modifier_builds_base_and_shifted_presses_in_order() {
        let modifier = Modifier::new([(b'a', b'A'), (b'1', b'!')]).unwrap();

        let result = modifier.supported_presses::<4>();

        assert_eq!(
            result,
            Ok([
                KeyPress { base: b'a', shifted: false },
                KeyPress { base: b'a', shifted: true },
                KeyPress { base: b'1', shifted: false },
                KeyPress { base: b'1', shifted: true },
            ])
        );
    }
}
