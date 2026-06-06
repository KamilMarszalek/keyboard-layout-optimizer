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
  const ratio = maxFreq > 0 ? freq / maxFreq : 0;

  // Interpolate from light neutral (low/no freq) to amber (high freq)
  const hue = 30;
  const saturation = Math.round(15 + ratio * 75); // 15 % → 90 %
  const lightness = Math.round(88 - ratio * 36); // 88 % → 52 %

  return { backgroundColor: `hsl(${hue}, ${saturation}%, ${lightness}%)` };
}
