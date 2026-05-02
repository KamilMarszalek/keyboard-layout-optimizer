use core::fmt;
use itertools::Itertools;

use super::common::{KEY_COUNT, KeyIndex};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Row {
    Number,
    Top,
    Home,
    Bottom,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Hand {
    Left,
    Right,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Hand {
    fn index(&self) -> usize {
        match self {
            Hand::Right => 0,
            Hand::Left => 1,
        }
    }
}

impl Hand {
    const COUNT: usize = 2;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Finger {
    Pinky,
    Ring,
    Middle,
    Index,
}

impl fmt::Display for Finger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Finger {
    fn index(&self) -> usize {
        match self {
            Finger::Pinky => 0,
            Finger::Ring => 1,
            Finger::Middle => 2,
            Finger::Index => 3,
        }
    }
}

impl Finger {
    const COUNT: usize = 4;
}

const N_FINGERS: usize = Hand::COUNT * Finger::COUNT;

#[derive(Clone, Copy, Debug)]
struct FingerCount {
    finger: Finger,
    count: usize,
    rest_at: Option<usize>,
}

macro_rules! fc {
    ( $finger:expr, $count:expr ) => {
        FingerCount { finger: $finger, count: $count, rest_at: None }
    };

    ( $finger:expr, $count:expr, $rest_at:expr ) => {
        FingerCount { finger: $finger, count: $count, rest_at: Some($rest_at) }
    };
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Coordinates {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Key {
    coords: Coordinates,
    hand: Hand,
    finger: Finger,
    row: Row,
    is_resting_key: bool,
}

struct RowSpec {
    left: Vec<FingerCount>,
    right: Vec<FingerCount>,
    x_offset: f32,
    y: f32,
    row: Row,
}

impl RowSpec {
    fn build_row(&self) -> Vec<Key> {
        let mut keys = Vec::with_capacity(self.size());
        let mut x = self.x_offset;

        let mut push_side = |keys: &mut Vec<Key>, side: &Vec<FingerCount>, hand: Hand| {
            for fc in side {
                for j in 0..fc.count {
                    keys.push(Key {
                        coords: Coordinates { x, y: self.y },
                        hand,
                        finger: fc.finger,
                        row: self.row,
                        is_resting_key: fc.rest_at == Some(j),
                    });
                    x += 1.0;
                }
            }
        };

        push_side(&mut keys, &self.left, Hand::Left);
        push_side(&mut keys, &self.right, Hand::Right);

        keys
    }

    fn size(&self) -> usize {
        let left_size = self.left.iter().map(|fc| fc.count).sum::<usize>();
        let right_size = self.right.iter().map(|fc| fc.count).sum::<usize>();
        left_size + right_size
    }

    fn used_fingers(&self) -> impl IntoIterator<Item = (Hand, Finger)> {
        let left = self.left.iter().map(|fc| (Hand::Left, fc.finger));
        let right = self.right.iter().map(|fc| (Hand::Right, fc.finger));
        left.chain(right)
    }
}

/// Physical description of a keyboard with `N` keys.
///
/// Each key stores its row, hand, finger assignment, and approximate 2D position. `Geometry` does
/// not define which symbols appear on those keys; it only describes the keyboard's physical
/// structure. Keys are ordered from left to right within a row, and from top to bottom across rows.
pub struct Geometry<const N: usize> {
    keys: [Key; N],
    default_placement: [Option<KeyIndex>; N_FINGERS],
}

impl<const N: usize> Geometry<N> {
    // Build a geometry keyboard from ordered row specifications.
    //
    /// Each `RowSpec` expands into a contiguous row of keys. The resulting key array preserves the
    /// order of the provided rows, and the order of keys implied by each row's `left` and `right`
    /// finger definitions.
    fn new<I>(specs: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = RowSpec>,
    {
        let collected: Vec<RowSpec> = specs.into_iter().collect();
        let n_fingers = collected.iter().flat_map(|f| f.used_fingers()).unique().count();

        let keys = Self::build_keys(&collected)?;
        let default_placement = Self::extract_default_placements(&keys, n_fingers)?;

        Ok(Self { keys, default_placement })
    }

    fn build_keys(specs: &[RowSpec]) -> Result<[Key; N], String> {
        let mut total = 0;
        let mut keys_vec = Vec::with_capacity(N);

        for spec in specs {
            total += spec.size();
            keys_vec.extend(spec.build_row());
        }

        match total == N {
            true => Ok(keys_vec.try_into().unwrap()),
            false => Err(format!("Specs must define exactly {} keys", N)),
        }
    }

    fn extract_default_placements(
        keys: &[Key; N],
        n_fingers: usize,
    ) -> Result<[Option<KeyIndex>; N_FINGERS], String> {
        let mut default_placement: [Option<KeyIndex>; N_FINGERS] = [None; N_FINGERS];
        for (i, key) in keys.iter().enumerate().filter(|(_, key)| key.is_resting_key) {
            let slot = key.hand.index() * Finger::COUNT + key.finger.index();
            if default_placement[slot].is_some() {
                return Err(format!(
                    "{}-{} has been already assigned to key",
                    key.hand, key.finger
                ));
            }
            default_placement[slot] = Some(i);
        }

        let filled = default_placement.iter().filter(|&k| k.is_some()).count();
        match filled == n_fingers {
            true => Ok(default_placement),
            false => Err(format!(
                "Only {} of {} possible key-finger assignments filled",
                filled, n_fingers
            )),
        }
    }

    pub fn default_key(&self, hand: Hand, finger: Finger) -> Option<&Key> {
        let i = hand.index() * Finger::COUNT + finger.index();
        match self.default_placement[i] {
            Some(i) => Some(&self.keys[i]),
            None => None,
        }
    }
}

impl Geometry<KEY_COUNT> {
    // Builds US ANSI-like geometry, containing `KEY_COUNT` keys that store visible ASCII symbols
    // ordered in 4 rows.
    pub fn standard_us() -> Self {
        let specs = [
            RowSpec {
                left: vec![
                    fc!(Finger::Pinky, 2),
                    fc!(Finger::Ring, 1),
                    fc!(Finger::Middle, 1),
                    fc!(Finger::Index, 2),
                ],
                right: vec![
                    fc!(Finger::Index, 2),
                    fc!(Finger::Middle, 1),
                    fc!(Finger::Ring, 1),
                    fc!(Finger::Pinky, 3),
                ],
                x_offset: 0.0,
                y: 0.0,
                row: Row::Number,
            },
            RowSpec {
                left: vec![
                    fc!(Finger::Pinky, 1),
                    fc!(Finger::Ring, 1),
                    fc!(Finger::Middle, 1),
                    fc!(Finger::Index, 2),
                ],
                right: vec![
                    fc!(Finger::Index, 2),
                    fc!(Finger::Middle, 1),
                    fc!(Finger::Ring, 1),
                    fc!(Finger::Pinky, 4),
                ],
                x_offset: 1.5,
                y: 1.0,
                row: Row::Top,
            },
            RowSpec {
                left: vec![
                    fc!(Finger::Pinky, 1, 0),
                    fc!(Finger::Ring, 1, 0),
                    fc!(Finger::Middle, 1, 0),
                    fc!(Finger::Index, 2, 0),
                ],
                right: vec![
                    fc!(Finger::Index, 2, 1),
                    fc!(Finger::Middle, 1, 0),
                    fc!(Finger::Ring, 1, 0),
                    fc!(Finger::Pinky, 2, 0),
                ],
                x_offset: 2.0,
                y: 2.0,
                row: Row::Home,
            },
            RowSpec {
                left: vec![
                    fc!(Finger::Pinky, 1),
                    fc!(Finger::Ring, 1),
                    fc!(Finger::Middle, 1),
                    fc!(Finger::Index, 2),
                ],
                right: vec![
                    fc!(Finger::Index, 2),
                    fc!(Finger::Middle, 1),
                    fc!(Finger::Ring, 1),
                    fc!(Finger::Pinky, 1),
                ],
                x_offset: 2.5,
                y: 3.0,
                row: Row::Bottom,
            },
        ];

        Self::new(specs).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_row_spec() -> RowSpec {
        RowSpec { left: vec![], right: vec![], x_offset: 0.0, y: 0.0, row: Row::Home }
    }

    #[test]
    fn spec_total_size() {
        let spec = RowSpec {
            left: vec![fc!(Finger::Pinky, 2)],
            right: vec![fc!(Finger::Pinky, 2)],
            ..test_row_spec()
        };
        assert_eq!(spec.size(), 4);
    }

    #[test]
    fn spec_build_row_from_left_to_right() {
        let spec = RowSpec {
            left: vec![
                fc!(Finger::Pinky, 1),
                fc!(Finger::Ring, 1),
                fc!(Finger::Middle, 1),
                fc!(Finger::Index, 1),
            ],
            right: vec![
                fc!(Finger::Pinky, 1),
                fc!(Finger::Ring, 1),
                fc!(Finger::Middle, 1),
                fc!(Finger::Index, 1),
            ],
            ..test_row_spec()
        };
        let order = [Finger::Pinky, Finger::Ring, Finger::Middle, Finger::Index];
        let n = order.len();
        let keys = spec.build_row();

        for (i, key) in keys.iter().enumerate() {
            let expected_finger = order[i % n];
            if i < n {
                assert_eq!(key.hand, Hand::Left);
            } else {
                assert_eq!(key.hand, Hand::Right);
            }
            assert_eq!(key.finger, expected_finger);
        }
    }

    #[test]
    fn spec_build_row_repeat_finger_count() {
        let spec = RowSpec { left: vec![fc!(Finger::Pinky, 5)], ..test_row_spec() };
        let keys = spec.build_row();
        for key in keys.iter() {
            assert_eq!(key.finger, Finger::Pinky);
        }
    }

    #[test]
    fn spec_build_row_increasing_x() {
        let spec = RowSpec {
            left: vec![
                fc!(Finger::Pinky, 1),
                fc!(Finger::Ring, 1),
                fc!(Finger::Middle, 1),
                fc!(Finger::Index, 1),
            ],
            right: vec![
                fc!(Finger::Pinky, 1),
                fc!(Finger::Ring, 1),
                fc!(Finger::Middle, 1),
                fc!(Finger::Index, 1),
            ],
            ..test_row_spec()
        };
        let keys = spec.build_row();
        for i in 1..keys.len() {
            assert_eq!(keys[i].coords.x - keys[i - 1].coords.x, 1.0);
        }
    }

    #[test]
    fn spec_build_row_setups_y() {
        let spec = RowSpec { left: vec![fc!(Finger::Pinky, 5)], y: 1.0, ..test_row_spec() };
        let keys = spec.build_row();
        for key in keys.iter() {
            assert_eq!(key.coords.y, 1.0);
        }
    }

    #[test]
    fn spec_build_row_setups_row() {
        let spec =
            RowSpec { left: vec![fc!(Finger::Pinky, 5)], row: Row::Bottom, ..test_row_spec() };
        let keys = spec.build_row();
        for key in keys.iter() {
            assert_eq!(key.row, Row::Bottom);
        }
    }

    #[test]
    fn spec_build_row_setups_x_offset() {
        let spec = RowSpec { left: vec![fc!(Finger::Pinky, 1)], x_offset: 1.0, ..test_row_spec() };
        let keys = spec.build_row();
        assert_eq!(keys[0].coords.x, 1.0);
    }

    #[test]
    fn geometry_new_succeeds_when_total_matches_n() {
        let specs = [
            RowSpec {
                left: vec![fc!(Finger::Pinky, 1, 0)],
                right: vec![fc!(Finger::Index, 1, 0)],
                row: Row::Top,
                ..test_row_spec()
            },
            RowSpec {
                left: vec![fc!(Finger::Ring, 1, 0)],
                right: vec![fc!(Finger::Middle, 1, 0)],
                row: Row::Home,
                ..test_row_spec()
            },
        ];
        let geometry = Geometry::<4>::new(specs);
        assert!(geometry.is_ok());
    }

    #[test]
    fn geometry_new_fails_when_total_is_less_than_n() {
        let specs = [RowSpec {
            left: vec![fc!(Finger::Pinky, 1, 0)],
            right: vec![fc!(Finger::Index, 1, 0)],
            ..test_row_spec()
        }];
        let geometry = Geometry::<3>::new(specs);
        assert_eq!(geometry.err().unwrap(), "Specs must define exactly 3 keys");
    }

    #[test]
    fn geometry_new_fails_when_total_is_greater_than_n() {
        let specs = [RowSpec {
            left: vec![fc!(Finger::Pinky, 2)],
            right: vec![fc!(Finger::Index, 2)],
            ..test_row_spec()
        }];
        let geometry = Geometry::<3>::new(specs);
        assert_eq!(geometry.err().unwrap(), "Specs must define exactly 3 keys");
    }

    #[test]
    fn geometry_new_fails_when_finger_is_not_assigned_once() {
        let specs = [
            RowSpec { left: vec![fc!(Finger::Pinky, 1, 0)], ..test_row_spec() },
            RowSpec { left: vec![fc!(Finger::Pinky, 1, 0)], ..test_row_spec() },
        ];
        let geometry = Geometry::<2>::new(specs);
        assert_eq!(
            geometry.err().unwrap(),
            format!("{}-{} has been already assigned to key", Hand::Left, Finger::Pinky),
        );
    }

    #[test]
    fn geometry_new_fails_when_finger_has_no_default_placement() {
        let specs = [RowSpec {
            left: vec![fc!(Finger::Pinky, 1, 0), fc!(Finger::Ring, 1)],
            ..test_row_spec()
        }];
        let geometry = Geometry::<2>::new(specs);
        assert_eq!(
            geometry.err().unwrap(),
            format!("Only {} of {} possible key-finger assignments filled", 1, 2),
        );
    }

    #[test]
    fn geometry_new_preserves_row_order_across_specs() {
        let specs = [
            RowSpec {
                left: vec![fc!(Finger::Pinky, 1, 0)],
                row: Row::Top,
                y: 1.0,
                ..test_row_spec()
            },
            RowSpec {
                left: vec![fc!(Finger::Ring, 1, 0)],
                row: Row::Home,
                y: 2.0,
                ..test_row_spec()
            },
        ];
        let geometry = Geometry::<2>::new(specs).unwrap();
        assert_eq!(geometry.keys[0].row, Row::Top);
        assert_eq!(geometry.keys[0].coords.y, 1.0);
        assert_eq!(geometry.keys[1].row, Row::Home);
        assert_eq!(geometry.keys[1].coords.y, 2.0);
    }

    #[test]
    fn geometry_new_assigns_default_placement_to_correct_finger() {
        let specs = [RowSpec {
            left: vec![fc!(Finger::Pinky, 1, 0)],
            right: vec![fc!(Finger::Index, 1, 0)],
            ..test_row_spec()
        }];
        let geometry = Geometry::<2>::new(specs).unwrap();

        let left_home = geometry.default_key(Hand::Left, Finger::Pinky).unwrap();
        assert_eq!(left_home.hand, Hand::Left);
        assert_eq!(left_home.finger, Finger::Pinky);
        assert_eq!(left_home.coords.x, 0.0);

        let right_home = geometry.default_key(Hand::Right, Finger::Index).unwrap();
        assert_eq!(right_home.hand, Hand::Right);
        assert_eq!(right_home.finger, Finger::Index);
        assert_eq!(right_home.coords.x, 1.0);
    }

    #[test]
    fn geometry_new_assigns_none_within_not_defined_hand() {
        let specs = [RowSpec { left: vec![fc!(Finger::Pinky, 1, 0)], ..test_row_spec() }];
        let geometry = Geometry::<1>::new(specs).unwrap();
        assert!(geometry.default_key(Hand::Right, Finger::Pinky).is_none());
    }

    #[test]
    fn geometry_new_assigns_none_within_not_defined_finger() {
        let specs = [RowSpec { left: vec![fc!(Finger::Pinky, 1, 0)], ..test_row_spec() }];
        let geometry = Geometry::<1>::new(specs).unwrap();
        assert!(geometry.default_key(Hand::Left, Finger::Index).is_none());
    }

    #[test]
    fn geometry_new_assigns_default_placement_at_correct_index_within_finger() {
        let specs = [RowSpec { left: vec![fc!(Finger::Pinky, 3, 2)], ..test_row_spec() }];
        let geometry = Geometry::<3>::new(specs).unwrap();
        let home_key = geometry.default_key(Hand::Left, Finger::Pinky).unwrap();
        assert_eq!(home_key.coords.x, 2.0);
    }

    #[test]
    fn geometry_standard_us_produces_key_count_keys() {
        let geometry = Geometry::standard_us();
        assert_eq!(geometry.keys.len(), KEY_COUNT);
    }

    #[test]
    fn geometry_standard_us_preserves_row_sizes() {
        let geometry = Geometry::standard_us();

        let row_size = |row| geometry.keys.iter().filter(|key| key.row == row).count();

        assert_eq!(row_size(Row::Number), 13);
        assert_eq!(row_size(Row::Top), 13);
        assert_eq!(row_size(Row::Home), 11);
        assert_eq!(row_size(Row::Bottom), 10);
    }
}
