use crate::keyboard::common::AsciiChar;

pub const US_KEY_COUNT: usize = 26 + 10 + 11; // letters + digits + punctuation
pub const US_PRESS_COUNT: usize = 2 * US_KEY_COUNT;

#[rustfmt::skip]
pub const QWERTY_US_SYMBOLS: [AsciiChar; US_KEY_COUNT] = [
    b'`', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=',
    b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\\',
    b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'',
    b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/',
];

#[rustfmt::skip]
pub const DVORAK_US_SYMBOLS: [AsciiChar; US_KEY_COUNT] = [
    b'`', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'[', b']',
    b'\'', b',', b'.', b'p', b'y', b'f', b'g', b'c', b'r', b'l', b'/', b'=', b'\\',
    b'a', b'o', b'e', b'u', b'i', b'd', b'h', b't', b'n', b's', b'-',
    b';', b'q', b'j', b'k', b'x', b'b', b'm', b'w', b'v', b'z',
];
