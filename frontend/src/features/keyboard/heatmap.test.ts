import { describe, expect, it } from 'vitest';

import { buildFrequencyMap, heatmapStyle, maxFrequency } from './heatmap';
import type { CharFrequency } from './types';

const freqs: CharFrequency[] = [
  { key: 'a', frequency: 10 },
  { key: 'b', frequency: 4 },
  { key: 'c', frequency: 0 },
];

describe('buildFrequencyMap', () => {
  it('maps each key to its frequency', () => {
    const map = buildFrequencyMap(freqs);
    expect(map.get('a')).toBe(10);
    expect(map.get('b')).toBe(4);
    expect(map.get('c')).toBe(0);
    expect(map.size).toBe(3);
  });

  it('returns an empty map when freqs are undefined', () => {
    expect(buildFrequencyMap().size).toBe(0);
  });

  it('returns an empty map for an empty list', () => {
    expect(buildFrequencyMap([]).size).toBe(0);
  });

  it('keeps the last value for duplicate keys', () => {
    const map = buildFrequencyMap([
      { key: 'a', frequency: 1 },
      { key: 'a', frequency: 9 },
    ]);
    expect(map.get('a')).toBe(9);
  });
});

describe('maxFrequency', () => {
  it('returns the highest frequency', () => {
    expect(maxFrequency(freqs)).toBe(10);
  });

  it('falls back to 1 when freqs are undefined', () => {
    expect(maxFrequency()).toBe(1);
  });

  it('falls back to 1 for an empty list', () => {
    expect(maxFrequency([])).toBe(1);
  });
});

describe('heatmapStyle', () => {
  it('produces a green hue for zero frequency (coldest)', () => {
    expect(heatmapStyle(0, 10)).toEqual({
      color: 'black',
      backgroundColor: 'hsl(120, 70%, 78%)',
    });
  });

  it('produces a red hue at maximum frequency (hottest)', () => {
    expect(heatmapStyle(10, 10)).toEqual({
      color: 'black',
      backgroundColor: 'hsl(0, 70%, 60%)',
    });
  });

  it('interpolates hue and lightness for mid-range frequency', () => {
    // ratio = 0.5 -> hue 60, lightness round(78 - 9) = 69
    expect(heatmapStyle(5, 10)).toEqual({
      color: 'black',
      backgroundColor: 'hsl(60, 70%, 69%)',
    });
  });

  it('guards against division by zero when maxFreq is 0', () => {
    // ratio coerced to 0 -> coldest style, no NaN in output
    expect(heatmapStyle(0, 0)).toEqual({
      color: 'black',
      backgroundColor: 'hsl(120, 70%, 78%)',
    });
  });
});
