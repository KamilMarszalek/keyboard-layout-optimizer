use crate::keyboard::{common::AsciiChar, model::KeyPress};

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
#[derive(Clone)]
pub struct Modifier {
    encode: HashMap<AsciiChar, AsciiChar>,
    decode: HashMap<AsciiChar, KeyPress>,
    symbols: Vec<AsciiChar>,
}

pub trait KeyPressMapper {
    fn supported_presses<const P: usize>(&self) -> Result<[KeyPress; P], SupportedPressesError>;
    fn key_press_of(&self, symbol: AsciiChar) -> Option<KeyPress>;
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

impl KeyPressMapper for Modifier {
    fn supported_presses<const P: usize>(&self) -> Result<[KeyPress; P], SupportedPressesError> {
        Modifier::supported_presses::<P>(self)
    }

    fn key_press_of(&self, symbol: AsciiChar) -> Option<KeyPress> {
        Modifier::key_press_of(self, symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyboard::common::KEY_COUNT;
    use rstest::rstest;

    #[rstest]
    #[case::letter_a(b'a', b'A')]
    #[case::letter_z(b'z', b'Z')]
    #[case::digit_1(b'1', b'!')]
    #[case::slash(b'/', b'?')]
    fn shift_supported(#[case] base: AsciiChar, #[case] shifted: AsciiChar) {
        let modifier =
            Modifier::new([(b'a', b'A'), (b'z', b'Z'), (b'1', b'!'), (b'/', b'?')]).unwrap();

        assert_eq!(modifier.shift(base).unwrap(), shifted);
    }

    #[rstest]
    #[case::shifted_base(b'B')]
    #[case::missing_base(b'/')]
    fn shift_unsupported(#[case] base: AsciiChar) {
        let modifier = Modifier::new([(b'a', b'A'), (b'1', b'!')]).unwrap();

        assert!(
            matches!(modifier.shift(base), Err(ModifierError::UnsupportedBase(found)) if found == base)
        );
    }

    #[test]
    fn base_symbols() {
        let modifier = Modifier::new([(b'a', b'A'), (b'1', b'!')]).unwrap();
        let mut symbols: Vec<AsciiChar> = modifier.base_symbols().to_vec();
        symbols.sort();
        assert_eq!(symbols, [b'1', b'a']);
    }

    #[rstest]
    #[case::base_letter(b'a', KeyPress { base: b'a', shifted: false })]
    #[case::shifted_letter(b'A', KeyPress { base: b'a', shifted: true })]
    #[case::base_digit(b'1', KeyPress { base: b'1', shifted: false })]
    #[case::shifted_digit(b'!', KeyPress { base: b'1', shifted: true })]
    #[case::base_punctuation(b'/', KeyPress { base: b'/', shifted: false })]
    #[case::shifted_punctuation(b'?', KeyPress { base: b'/', shifted: true })]
    fn key_press(#[case] symbol: AsciiChar, #[case] expected: KeyPress) {
        let modifier = Modifier::standard_us();

        assert_eq!(modifier.key_press_of(symbol), Some(expected));
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

    #[rstest]
    #[case::duplicate_base(vec![(b'a', b'A'), (b'a', b'@')], ModifierError::DuplicateBase(b'a'))]
    #[case::duplicate_shifted(
        vec![(b'a', b'!'), (b'1', b'!')],
        ModifierError::DuplicateShifted(b'!')
    )]
    #[case::base_also_shifted(
        vec![(b'a', b'A'), (b'A', b'!')],
        ModifierError::AmbiguousSymbol(b'A')
    )]
    #[case::base_matches_shifted(vec![(b'a', b'a')], ModifierError::AmbiguousSymbol(b'a'))]
    fn modifier_new_returns_error(
        #[case] shift_pairs: Vec<(AsciiChar, AsciiChar)>,
        #[case] expected: ModifierError,
    ) {
        let result = Modifier::new(shift_pairs);

        match result {
            Err(error) => assert_eq!(error, expected),
            Ok(_) => panic!("expected {expected:?}"),
        }
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

    #[test]
    fn supported_presses_returns_error_for_wrong_size() {
        let modifier = Modifier::new([(b'a', b'A'), (b'1', b'!')]).unwrap();

        let result = modifier.supported_presses::<3>();

        assert!(matches!(
            result,
            Err(SupportedPressesError::InvalidSupportedPressCount { expected: 3, actual: 4 })
        ));
    }
}
