/// Total number of physical keys in the main alphanumeric section of a US keyboard
/// (26 letters + 10 digits + 11 punctuation keys).
pub const US_KEY_COUNT: usize = 26 + 10 + 11;

/// Total number of logical key presses on a US keyboard, counting each key twice
/// for its unshifted and shifted output.
pub const US_PRESS_COUNT: usize = 2 * US_KEY_COUNT;
