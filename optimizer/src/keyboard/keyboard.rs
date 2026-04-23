#[derive(Clone, Copy)]
enum Row {
    Top,
    Home,
    Bottom,
}

#[derive(Clone, Copy)]
enum Hand {
    Left,
    Right,
}

#[derive(Clone, Copy)]
enum Finger {
    Pinky,
    Ring,
    Middle,
    Index,
    Thumb,
}

struct Coordinates {
    x: f32,
    y: f32,
}

struct Key {
    coords: Coordinates,
    hand: Hand,
    finger: Finger,
    row: Row,
}

struct KeyPress {
    letter: u8,
    shift: bool,
}

struct Keyboard {
    keys: Vec<Key>,
}

struct RowSpec<'a> {
    x_offset: f32,
    y: f32,
    refs: &'a [(Hand, Finger)],
    row: Row,
}

struct Layout {
    mappings: Vec<u8>,
}

impl Keyboard {
    pub fn from_geometry(specs: &Vec<RowSpec>) -> Self {
        let mut keys = Vec::new();
        for spec in specs {
            keys.extend(Self::build_row(spec));
        }
        Self { keys }
    }

    fn build_row(spec: &RowSpec) -> Vec<Key> {
        spec.refs
            .iter()
            .enumerate()
            .map(|(x, &(hand, finger))| Key {
                coords: Coordinates {
                    x: x as f32 + spec.x_offset,
                    y: spec.y,
                },
                hand: hand,
                finger: finger,
                row: spec.row,
            })
            .collect()
    }
}

fn standard_keyboard() -> Keyboard {
    let refs = [
        (Hand::Left, Finger::Pinky),
        (Hand::Left, Finger::Ring),
        (Hand::Left, Finger::Middle),
        (Hand::Left, Finger::Index),
        (Hand::Left, Finger::Index),
        (Hand::Right, Finger::Index),
        (Hand::Right, Finger::Index),
        (Hand::Right, Finger::Middle),
        (Hand::Right, Finger::Ring),
        (Hand::Right, Finger::Pinky),
    ];

    let spec = vec![
        RowSpec {
            x_offset: 0.0,
            y: 0.0,
            refs: &refs,
            row: Row::Top,
        },
        RowSpec {
            x_offset: 0.5,
            y: 1.0,
            refs: &refs,
            row: Row::Home,
        },
        RowSpec {
            x_offset: 1.0,
            y: 2.0,
            refs: &refs,
            row: Row::Bottom,
        },
    ];

    Keyboard::from_geometry(&spec)
}
