use any_ascii::any_ascii;

pub fn normalize_text(input: &str) -> String {
    any_ascii(input).chars().filter(|c| c.is_ascii_graphic()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_text_keeps_only_ascii_letters_and_lowercases() {
        let result = normalize_text("Ąłą ma Kota! 123");
        assert_eq!(result, "AlamaKota!123");
    }
}
