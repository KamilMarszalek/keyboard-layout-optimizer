import type { KeyMapping, Layout } from './types';

export const KEYBOARD_ROW_SIZES = [13, 13, 11, 10] as const;
export const KEYBOARD_ROW_OFFSETS = ['pl-0', 'pl-8', 'pl-16', 'pl-24'] as const;
export const EXPECTED_LAYOUT_LENGTH = KEYBOARD_ROW_SIZES.reduce((sum, size) => sum + size, 0);

export function hasExpectedLayoutLength(layout: Layout): boolean {
  return layout.mappings.length === EXPECTED_LAYOUT_LENGTH;
}

export function layoutToRows(
  layout: Layout,
  rowSizes: readonly number[] = KEYBOARD_ROW_SIZES,
): KeyMapping[][] {
  let start = 0;

  return rowSizes.map((size) => {
    const row = layout.mappings.slice(start, start + size);
    start += size;
    return row;
  });
}
