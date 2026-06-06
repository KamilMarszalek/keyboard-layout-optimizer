import type { CharFrequency } from './types';

export type KeyHeatStyle = {
  backgroundColor: string;
};

export function buildFrequencyMap(freqs?: CharFrequency[]): Map<string, number> {
  if (!freqs) return new Map();
  return new Map(freqs.map(({ key, frequency }) => [key, frequency]));
}

export function maxFrequency(freqs?: CharFrequency[]): number {
  if (!freqs || freqs.length === 0) return 1;
  return Math.max(...freqs.map((f) => f.frequency));
}

export function keyHeatStyle(
  base: string,
  freqMap: Map<string, number>,
  maxFreq: number,
): KeyHeatStyle {
  const freq = freqMap.get(base) ?? 0;
  if (freq == 0) {
    return { backgroundColor: 'background-color' };
  }

  const ratio = maxFreq > 0 ? freq / maxFreq : 0;
  const hue = Math.round(120 * (1 - ratio));
  const saturation = 70;
  const lightness = Math.round(78 - ratio * 18);

  return { backgroundColor: `hsl(${hue}, ${saturation}%, ${lightness}%)` };
}
