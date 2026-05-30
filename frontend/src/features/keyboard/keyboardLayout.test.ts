import { describe, expect, it } from 'vitest';
import {
  EXPECTED_LAYOUT_LENGTH,
  hasExpectedLayoutLength,
  layoutToRows,
} from './keyboardLayout';

describe('keyboard layout helpers', () => {
  it('uses the expected physical layout length', () => {
    expect(EXPECTED_LAYOUT_LENGTH).toBe(47);
  });

  it('splits a flat layout into physical keyboard rows', () => {
    const layout = Array.from({ length: EXPECTED_LAYOUT_LENGTH }, (_, index) => String(index));

    expect(layoutToRows(layout).map((row) => row.length)).toEqual([13, 13, 11, 10]);
    expect(layoutToRows(layout)[0][0]).toBe('0');
    expect(layoutToRows(layout)[3][9]).toBe('46');
  });

  it('validates layout length', () => {
    expect(hasExpectedLayoutLength(Array.from({ length: EXPECTED_LAYOUT_LENGTH }, () => ''))).toBe(
      true,
    );
    expect(
      hasExpectedLayoutLength(Array.from({ length: EXPECTED_LAYOUT_LENGTH - 1 }, () => '')),
    ).toBe(false);
  });
});
