const STANDARD_KEYBOARD_ROWS = [
  ["`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "="],
  ["q", "w", "e", "r", "t", "y", "u", "i", "o", "p", "[", "]", "\\"],
  ["a", "s", "d", "f", "g", "h", "j", "k", "l", ";", "'"],
  ["z", "x", "c", "v", "b", "n", "m", ",", ".", "/"],
] as const;

export const KEYBOARD_ROW_SIZES = STANDARD_KEYBOARD_ROWS.map((row) => row.length);
export const KEYBOARD_ROW_OFFSETS = ["pl-0", "pl-8", "pl-16", "pl-24"] as const;
export const EXPECTED_LAYOUT_LENGTH = KEYBOARD_ROW_SIZES.reduce((sum, size) => sum + size, 0);

export function standardKeyboardRows(): string[][] {
  return STANDARD_KEYBOARD_ROWS.map((row) => [...row]);
}

export function flattenKeyboardRows(rows: readonly (readonly string[])[]): string[] {
  return rows.flat();
}

export function layoutToRows(
  layout: readonly string[],
  rowSizes: readonly number[] = KEYBOARD_ROW_SIZES,
): string[][] {
  let start = 0;

  return rowSizes.map((size) => {
    const row = layout.slice(start, start + size);
    start += size;
    return row;
  });
}
