import type { MetricBreakdown } from './results.types';

type MetricLabel = {
  key: keyof MetricBreakdown;
  label: string;
};

export const metricLabels: ReadonlyArray<MetricLabel> = [
  {
    key: 'sameFingerBigrams',
    label: 'Same finger bigrams',
  },
  {
    key: 'fingerDistance',
    label: 'Finger distance',
  },
  {
    key: 'homeRowUsage',
    label: 'Home row usage',
  },
  {
    key: 'handAlternation',
    label: 'Hand alternation',
  },
  {
    key: 'rowJumping',
    label: 'Row jumping',
  },
];
