export const KEYBOARD_ROW_SIZES = [13, 13, 11, 10] as const;
export const KEYBOARD_ROW_OFFSETS = ['pl-0', 'pl-8', 'pl-16', 'pl-24'] as const;
export const EXPECTED_LAYOUT_LENGTH = KEYBOARD_ROW_SIZES.reduce((sum, size) => sum + size, 0);

export function hasExpectedLayoutLength(layout: readonly string[]): boolean {
  return layout.length === EXPECTED_LAYOUT_LENGTH;
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
