use any_ascii::any_ascii;

use crate::keyboard::{model::KeyPress, modifier::KeyPressMapper};

/// Normalizes `input` text
///
/// It transliterates non-ASCII characters to their ASCII equivalents
/// and removes non-graphical characters.
pub fn normalize_text(input: &str) -> String {
    any_ascii(input).chars().filter(|c| c.is_ascii_graphic()).collect()
}

pub fn map_normalized_text_to_key_presses(
    normalized: &str,
    mapper: &impl KeyPressMapper,
) -> impl Iterator<Item = Option<KeyPress>> {
    normalized.bytes().map(|c| mapper.key_press_of(c))
}

#[cfg(test)]
mod tests {
    use crate::keyboard::modifier::Modifier;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::transliterates_non_ascii_characters("Ąłą", "Ala")]
    #[case::preserves_case_digits_and_punctuation_but_removes_spaces("ma Kota! 123", "maKota!123")]
    fn normalize_text_cases(#[case] input: &str, #[case] expected: &str) {
        let result = normalize_text(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::supported_symbols(
        Modifier::new([(b'a', b'A'), (b'1', b'!')]).unwrap(),
        "aA1!",
        vec![
            Some(KeyPress { base: b'a', shifted: false }),
            Some(KeyPress { base: b'a', shifted: true }),
            Some(KeyPress { base: b'1', shifted: false }),
            Some(KeyPress { base: b'1', shifted: true }),
        ]
    )]
    #[case::unsupported_symbols_reset_to_none(
        Modifier::new([(b'a', b'A'), (b'b', b'B')]).unwrap(),
        "a b",
        vec![
            Some(KeyPress { base: b'a', shifted: false }),
            None,
            Some(KeyPress { base: b'b', shifted: false }),
        ]
    )]
    fn map_normalized_text_to_key_presses_cases(
        #[case] modifier: Modifier,
        #[case] input: &str,
        #[case] expected: Vec<Option<KeyPress>>,
    ) {
        let result: Vec<_> = map_normalized_text_to_key_presses(input, &modifier).collect();

        assert_eq!(result, expected);
    }
}
