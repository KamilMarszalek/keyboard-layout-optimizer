pub fn normalize_text(input: &str) -> String {
    input.chars().filter(|c| c.is_ascii_alphabetic()).map(|c| c.to_ascii_lowercase()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_text_keeps_only_ascii_letters_and_lowercases() {
        let result = normalize_text("Ala ma Kota! 123");
        assert_eq!(result, "alamakota");
    }
}
