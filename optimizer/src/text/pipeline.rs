use any_ascii::any_ascii;

use crate::keyboard::{model::KeyPress, modifier::Modifier};

/// Normalizes `input` text
///
/// It transliterates non-ASCII characters to their ASCII equivalents
/// and removes non-graphical characters.
pub fn normalize_text(input: &str) -> String {
    any_ascii(input).chars().filter(|c| c.is_ascii_graphic()).collect()
}

pub fn map_normalized_text_to_key_presses(
    normalized: &str,
    modifier: &Modifier,
) -> impl Iterator<Item = Option<KeyPress>> {
    normalized.bytes().map(|c| modifier.key_press_of(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_text_transliterates_non_ascii_characters() {
        let result = normalize_text("Ąłą");
        assert_eq!(result, "Ala");
    }

    #[test]
    fn normalize_text_preserves_case_digits_and_punctuation_but_removes_spaces() {
        let result = normalize_text("ma Kota! 123");
        assert_eq!(result, "maKota!123");
    }
}

#[test]
fn map_normalized_text_to_key_presses_maps_supported_symbols() {
    let modifier = Modifier::new([(b'a', b'A'), (b'1', b'!')]).unwrap();

    let result: Vec<_> = map_normalized_text_to_key_presses("aA1!", &modifier).collect();

    assert_eq!(
        result,
        vec![
            Some(KeyPress { base: b'a', shifted: false }),
            Some(KeyPress { base: b'a', shifted: true }),
            Some(KeyPress { base: b'1', shifted: false }),
            Some(KeyPress { base: b'1', shifted: true }),
        ]
    );
}

#[test]
fn map_normalized_text_to_key_presses_returns_none_for_unsupported_symbols() {
    let modifier = Modifier::new([(b'a', b'A'), (b'b', b'B')]).unwrap();

    let result: Vec<_> = map_normalized_text_to_key_presses("a b", &modifier).collect();

    assert_eq!(
        result,
        vec![
            Some(KeyPress { base: b'a', shifted: false }),
            None,
            Some(KeyPress { base: b'b', shifted: false }),
        ]
    );
}
