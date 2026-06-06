import type { NumberControlField } from '@/lib/field';

import { METRIC_WEIGHT_MAX, METRIC_WEIGHT_MIN } from './constants';
import type { AnnealingParams, Config, MetricsWeights } from './schema';

export const metricControls: ReadonlyArray<NumberControlField<MetricsWeights>> = [
  {
    key: 'sameFingerBigrams',
    label: 'Same finger bigrams',
    description: 'Penalty for repeated use of the same finger on adjacent letters.',
    step: 0.1,
    min: METRIC_WEIGHT_MIN,
    max: METRIC_WEIGHT_MAX,
  },
  {
    key: 'fingerDistance',
    label: 'Finger distance',
    description: 'Penalty for longer finger travel across the keyboard.',
    step: 0.1,
    min: METRIC_WEIGHT_MIN,
    max: METRIC_WEIGHT_MAX,
  },
  {
    key: 'homeRowUsage',
    label: 'Home row usage',
    description: 'Reward for keeping common keys near the home row.',
    step: 0.1,
    min: METRIC_WEIGHT_MIN,
    max: METRIC_WEIGHT_MAX,
  },
  {
    key: 'handAlternation',
    label: 'Hand alternation',
    description: 'Reward for alternating between hands while typing.',
    step: 0.1,
    min: METRIC_WEIGHT_MIN,
    max: METRIC_WEIGHT_MAX,
  },
  {
    key: 'rowJumping',
    label: 'Row jumping',
    description: 'Penalty for movement between distant keyboard rows.',
    step: 0.1,
    min: METRIC_WEIGHT_MIN,
    max: METRIC_WEIGHT_MAX,
  },
];

export const annealingControls: ReadonlyArray<NumberControlField<AnnealingParams>> = [
  {
    key: 'tStart',
    label: 'Start temperature',
    step: 0.1,
    min: 0.001,
  },
  {
    key: 'tMin',
    label: 'Minimum temperature',
    step: 0.001,
    min: 0.001,
  },
  {
    key: 'alpha',
    label: 'Cooling alpha',
    step: 0.001,
    min: 0,
    max: 0.999,
  },
  {
    key: 'iterationsPerTemp',
    label: 'Iterations per temperature',
    step: 1,
    min: 1,
  },
];

export const seedControl: NumberControlField<Config> = {
  key: 'seed',
  label: 'Seed',
  description:
    'Fixed seed for reproducible runs. Leaving this blank uses seed 42; every unseeded run produces identical results.',
  step: 1,
};
