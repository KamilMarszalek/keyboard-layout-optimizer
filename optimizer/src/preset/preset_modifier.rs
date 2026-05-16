use crate::keyboard::modifier::Modifier;

impl Modifier {
    /// Returns the US ASCII Shift mapping for the main printable keyboard symbols.
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
            .expect("US ASCII modifier mapping is valid")
    }
}

#[cfg(test)]
mod tests {
    use crate::preset::constants::US_KEY_COUNT;

    use super::*;
    #[test]
    fn us_ascii() {
        let modifier = Modifier::standard_us();
        assert_eq!(modifier.base_symbols().len(), US_KEY_COUNT);
        for &symbol in modifier.base_symbols().iter() {
            modifier.shift(symbol).unwrap();
        }
    }
}
