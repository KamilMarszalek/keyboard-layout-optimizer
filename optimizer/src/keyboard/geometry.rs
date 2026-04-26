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

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct Coordinates {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Key {
    coords: Coordinates,
    hand: Hand,
    finger: Finger,
    row: Row,
}

pub struct RowSpec {
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
        }

        keys
    }

    fn size(&self) -> usize {
        let left_size = self.left.iter().map(|fc| fc.count).sum::<usize>();
        let right_size = self.right.iter().map(|fc| fc.count).sum::<usize>();
        left_size + right_size
    }
}

#[allow(dead_code)]
pub struct Geometry<const N: usize> {
    keys: [Key; N],
}

impl<const N: usize> Geometry<N> {
    // keys are created from left to right, from top to bottom,
    // specs 'left' and 'right' must preserve the order
    pub fn new<I>(specs: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = RowSpec>,
    {
        let mut total = 0;
        let mut keys_vec = Vec::with_capacity(N);

        for spec in specs {
            total += spec.size();
            keys_vec.extend(spec.build_row());
        }

        if total != N {
            return Err(format!("Specs must define exactly {} keys", N).to_string());
        }

        let keys: [Key; N] = keys_vec.try_into().unwrap();

        Ok(Self { keys })
    }
}

impl Geometry<KEY_COUNT> {
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
                right: [fc!(Finger::Index, 2), fc!(Finger::Middle, 1), fc!(Finger::Ring, 1), fc!(Finger::Pinky, 2)],
                x_offset: 2.0,
                y: 2.0,
                row: Row::Home,
            },
            RowSpec {
                left: [fc!(Finger::Pinky, 1), fc!(Finger::Ring, 1), fc!(Finger::Middle, 1), fc!(Finger::Index, 2)],
                right: [fc!(Finger::Index, 2), fc!(Finger::Middle, 1), fc!(Finger::Ring, 1), fc!(Finger::Pinky, 1)],
                x_offset: 2.5,
                y: 3.0,
                row: Row::Bottom,
            },
        ];

        Self::new(specs).unwrap()
    }
}
