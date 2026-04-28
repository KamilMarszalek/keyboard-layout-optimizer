use any_ascii::any_ascii;

pub fn normalize_text(input: &str) -> String {
    any_ascii(input).chars().filter(|c| c.is_ascii_graphic()).collect()
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
