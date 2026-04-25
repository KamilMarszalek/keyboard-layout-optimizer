use super::common::KEY_COUNT;

#[derive(Clone, Copy, Debug)]
enum Row {
    Number,
    Top,
    Home,
    Bottom,
}

#[derive(Clone, Copy, Debug)]
enum Hand {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum Finger {
    Pinky,
    Ring,
    Middle,
    Index,
}

#[derive(Clone, Copy, Debug)]
struct FingerCount {
    finger: Finger,
    count: usize,
}

macro_rules! fc {
    ( $finger:expr, $count:expr ) => {
        FingerCount { finger: $finger, count: $count }
    };
}

#[derive(Clone, Copy, Debug)]
struct Coordinates {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Key {
    coords: Coordinates,
    hand: Hand,
    finger: Finger,
    row: Row,
}

struct RowSpec {
    left: [FingerCount; 4],
    right: [FingerCount; 4],
    x_offset: f32,
    y: f32,
    row: Row,
}

impl RowSpec {
    fn build_row(&self) -> Vec<Key> {
        let mut keys = Vec::with_capacity(self.left.len() + self.right.len());
        let mut base_x = 0.0;

        for (i, &finger) in self.left.iter().flat_map(|fc| std::iter::repeat_n(&fc.finger, fc.count)).enumerate() {
            keys.push(Key {
                coords: Coordinates { x: i as f32 + self.x_offset, y: self.y },
                hand: Hand::Left,
                finger,
                row: self.row,
            });
            base_x += 1.0;
        }

        for (i, &finger) in self.right.iter().flat_map(|fc| std::iter::repeat_n(&fc.finger, fc.count)).enumerate() {
            keys.push(Key {
                coords: Coordinates { x: base_x + i as f32 + self.x_offset, y: self.y },
                hand: Hand::Right,
                finger,
                row: self.row,
            });
            base_x += 1.0
        }

        keys
    }

    fn size(&self) -> usize {
        let left_size = self.left.iter().map(|fc| fc.count).sum::<usize>();
        let right_size = self.right.iter().map(|fc| fc.count).sum::<usize>();
        left_size + right_size
    }
}

pub struct Geometry {
    keys: [Key; KEY_COUNT],
}

impl Geometry {
    // keys are created from left to right, from top to bottom,
    // specs 'left' and 'right' must preserve the order
    pub fn new(specs: &[RowSpec]) -> Result<Self, String> {
        if specs.iter().map(|s| s.size()).sum::<usize>() != KEY_COUNT {
            return Err(format!("Specs must define exactly {} keys", KEY_COUNT).to_string());
        }

        let mut keys_vec = Vec::with_capacity(KEY_COUNT);
        for spec in specs {
            keys_vec.extend(spec.build_row())
        }

        let keys: [Key; KEY_COUNT] = keys_vec.try_into().unwrap();

        Ok(Self { keys })
    }

    pub fn standard_us() -> Self {
        let specs = [
            RowSpec {
                left: [fc!(Finger::Pinky, 2), fc!(Finger::Ring, 1), fc!(Finger::Middle, 1), fc!(Finger::Index, 2)],
                right: [fc!(Finger::Index, 2), fc!(Finger::Middle, 1), fc!(Finger::Ring, 1), fc!(Finger::Pinky, 3)],
                x_offset: 0.0,
                y: 0.0,
                row: Row::Number,
            },
            RowSpec {
                left: [fc!(Finger::Pinky, 1), fc!(Finger::Ring, 1), fc!(Finger::Middle, 1), fc!(Finger::Index, 2)],
                right: [fc!(Finger::Index, 2), fc!(Finger::Middle, 1), fc!(Finger::Ring, 1), fc!(Finger::Pinky, 4)],
                x_offset: 1.5,
                y: 1.0,
                row: Row::Top,
            },
            RowSpec {
                left: [fc!(Finger::Pinky, 1), fc!(Finger::Ring, 1), fc!(Finger::Middle, 1), fc!(Finger::Index, 2)],
                right: [fc!(Finger::Index, 2), fc!(Finger::Middle, 1), fc!(Finger::Ring, 1), fc!(Finger::Pinky, 4)],
                x_offset: 2.0,
                y: 2.0,
                row: Row::Home,
            },
            RowSpec {
                left: [fc!(Finger::Pinky, 1), fc!(Finger::Ring, 1), fc!(Finger::Middle, 1), fc!(Finger::Index, 2)],
                right: [fc!(Finger::Index, 2), fc!(Finger::Middle, 1), fc!(Finger::Ring, 1), fc!(Finger::Pinky, 1)],
                x_offset: 2.0,
                y: 2.0,
                row: Row::Bottom,
            },
        ];

        Self::new(&specs).unwrap()
    }
}
