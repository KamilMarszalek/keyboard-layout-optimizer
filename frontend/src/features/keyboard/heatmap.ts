import type { CharFrequency, KeyMapping } from './types';

export type KeyHeatStyle = {
  color: string;
  backgroundColor: string;
};

export type KeyView = {
  mapping: KeyMapping;
  style?: Partial<KeyHeatStyle>;
};

export function buildFrequencyMap(freqs?: CharFrequency[]): Map<string, number> {
  if (!freqs) return new Map();
  return new Map(freqs.map(({ key, frequency }) => [key, frequency]));
}

export function maxFrequency(freqs?: CharFrequency[]): number {
  if (!freqs || freqs.length === 0) return 1;
  return Math.max(...freqs.map((f) => f.frequency));
}

export function heatmapStyle(freq: number, maxFreq: number): KeyHeatStyle {
  const ratio = maxFreq > 0 ? freq / maxFreq : 0;
  const hue = Math.round(120 * (1 - ratio));
  const saturation = 70;
  const lightness = Math.round(78 - ratio * 18);
  return {
    color: 'black',
    backgroundColor: `hsl(${hue}, ${saturation}%, ${lightness}%)`,
  };
}
