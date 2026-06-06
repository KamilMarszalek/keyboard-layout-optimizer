import { describe, expect, it } from 'vitest';

import { EXPECTED_LAYOUT_LENGTH, hasExpectedLayoutLength, layoutToRows } from './keyboardLayout';
import type { Layout } from './types';

function makeLayout(length: number, base: (index: number) => string = String): Layout {
  return {
    mappings: Array.from({ length }, (_, index) => ({ base: base(index), shifted: '' })),
  };
}

describe('keyboard layout helpers', () => {
  it('uses the expected physical layout length', () => {
    expect(EXPECTED_LAYOUT_LENGTH).toBe(47);
  });

  it('splits a flat layout into physical keyboard rows', () => {
    const layout = makeLayout(EXPECTED_LAYOUT_LENGTH);

    expect(layoutToRows(layout).map((row) => row.length)).toEqual([13, 13, 11, 10]);
    expect(layoutToRows(layout)[0][0].base).toBe('0');
    expect(layoutToRows(layout)[3][9].base).toBe('46');
  });

  it('validates layout length', () => {
    expect(hasExpectedLayoutLength(makeLayout(EXPECTED_LAYOUT_LENGTH))).toBe(true);
    expect(hasExpectedLayoutLength(makeLayout(EXPECTED_LAYOUT_LENGTH - 1))).toBe(false);
  });
});
