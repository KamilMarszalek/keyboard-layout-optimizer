use super::constants::US_KEY_COUNT;
use crate::fc;
use crate::keyboard::geometry::{Finger, FingerCount, Geometry, Row, RowSpec};

impl Geometry<US_KEY_COUNT> {
    // Builds US ANSI-like geometry, containing `KEY_COUNT` keys that store visible ASCII symbols
    // ordered in 4 rows.
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
