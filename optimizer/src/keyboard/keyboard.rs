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
    x: u8,
    y: u8,
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
    key_to_letter: Vec<u8>,
    // left_shift: Key,
    // right_shift: Key,
}

fn row(n: u8, y: u8, refs: &[(Hand, Finger)], row: Row) -> Vec<Key> {
    (0..n)
        .map(|x| Key {
            coords: Coordinates { x: x, y: y },
            hand: refs[x as usize].0,
            finger: refs[x as usize].1,
            row: row,
        })
        .collect()
}

fn qwerty_keyboard() -> Keyboard {
    let mut keys = Vec::new();
    let layout = vec![
        b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'a', b's', b'd', b'f', b'g',
        b'h', b'j', b'k', b'l', b'z', b'x', b'c', b'v', b'b', b'n', b'm',
    ];

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

    let top = row(10, 0, &refs, Row::Top); // qwertyuiop
    let middle = row(9, 1, &refs, Row::Home); // asdfghjkl
    let bottom = row(7, 2, &refs, Row::Bottom); // zxcvbnm

    keys.extend(top);
    keys.extend(middle);
    keys.extend(bottom);

    Keyboard {
        keys: keys,
        key_to_letter: layout,
    }
}
