import type { AnnealingParams, MetricsWeights } from './config.schema';

type FieldDefinition<T> = {
  key: keyof T;
  label: string;
  description?: string;
};

type NumberFieldDefinition<T> = FieldDefinition<T> & {
  step: number;
  min?: number;
  max?: number;
};

export const metricControls: ReadonlyArray<NumberFieldDefinition<MetricsWeights>> = [
  {
    key: 'sameFingerBigrams',
    label: 'Same finger bigrams',
    description: 'Penalty for repeated use of the same finger on adjacent letters.',
    step: 0.1,
    min: 0,
    max: 5,
  },
  {
    key: 'fingerDistance',
    label: 'Finger distance',
    description: 'Penalty for longer finger travel across the keyboard.',
    step: 0.1,
    min: 0,
    max: 5,
  },
  {
    key: 'homeRowUsage',
    label: 'Home row usage',
    description: 'Reward for keeping common keys near the home row.',
    step: 0.1,
    min: 0,
    max: 5,
  },
  {
    key: 'handAlternation',
    label: 'Hand alternation',
    description: 'Reward for alternating between hands while typing.',
    step: 0.1,
    min: 0,
    max: 5,
  },
  {
    key: 'rowJumping',
    label: 'Row jumping',
    description: 'Penalty for movement between distant keyboard rows.',
    step: 0.1,
    min: 0,
    max: 5,
  },
];

export const annealingControls: ReadonlyArray<NumberFieldDefinition<AnnealingParams>> = [
  {
    key: 'tStart',
    label: 'Start temperature',
    step: 0.1,
    min: 0,
  },
  {
    key: 'tMin',
    label: 'Minimum temperature',
    step: 0.0001,
    min: 0,
  },
  {
    key: 'alpha',
    label: 'Cooling alpha',
    step: 0.0001,
    min: 0,
    max: 1,
  },
  {
    key: 'iterationsPerTemp',
    label: 'Iterations per temperature',
    step: 1,
    min: 0,
  },
];
