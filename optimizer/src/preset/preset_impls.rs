use super::constants::US_KEY_COUNT;
use crate::fc;
use crate::keyboard::geometry::{Finger, Geometry, Row, RowSpec};
use crate::keyboard::modifier::Modifier;

impl Geometry<US_KEY_COUNT> {
    /// Builds US ANSI-like geometry containing `US_KEY_COUNT` keys ordered in 4 rows
    /// (number, top, home, bottom) from left to right.
    pub fn standard_us() -> Self {
        let specs = [
            RowSpec::new(
                vec![
                    fc!(Finger::Pinky, 2),
                    fc!(Finger::Ring, 1),
                    fc!(Finger::Middle, 1),
                    fc!(Finger::Index, 2),
                ],
                vec![
                    fc!(Finger::Index, 2),
                    fc!(Finger::Middle, 1),
                    fc!(Finger::Ring, 1),
                    fc!(Finger::Pinky, 3),
                ],
                0.0,
                0.0,
                Row::Number,
            ),
            RowSpec::new(
                vec![
                    fc!(Finger::Pinky, 1),
                    fc!(Finger::Ring, 1),
                    fc!(Finger::Middle, 1),
                    fc!(Finger::Index, 2),
                ],
                vec![
                    fc!(Finger::Index, 2),
                    fc!(Finger::Middle, 1),
                    fc!(Finger::Ring, 1),
                    fc!(Finger::Pinky, 4),
                ],
                1.5,
                1.0,
                Row::Top,
            ),
            RowSpec::new(
                vec![
                    fc!(Finger::Pinky, 1, 0),
                    fc!(Finger::Ring, 1, 0),
                    fc!(Finger::Middle, 1, 0),
                    fc!(Finger::Index, 2, 0),
                ],
                vec![
                    fc!(Finger::Index, 2, 1),
                    fc!(Finger::Middle, 1, 0),
                    fc!(Finger::Ring, 1, 0),
                    fc!(Finger::Pinky, 2, 0),
                ],
                2.0,
                2.0,
                Row::Home,
            ),
            RowSpec::new(
                vec![
                    fc!(Finger::Pinky, 1),
                    fc!(Finger::Ring, 1),
                    fc!(Finger::Middle, 1),
                    fc!(Finger::Index, 2),
                ],
                vec![
                    fc!(Finger::Index, 2),
                    fc!(Finger::Middle, 1),
                    fc!(Finger::Ring, 1),
                    fc!(Finger::Pinky, 1),
                ],
                2.5,
                3.0,
                Row::Bottom,
            ),
        ];

        Self::new(specs).unwrap()
    }
}

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
    fn standard_us() {
        let modifier = Modifier::standard_us();
        assert_eq!(modifier.base_symbols().len(), US_KEY_COUNT);
        for &symbol in modifier.base_symbols() {
            modifier.shift(symbol).unwrap();
        }
    }
}
