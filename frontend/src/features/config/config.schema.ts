import { z } from 'zod';
import { METRIC_WEIGHT_MAX, METRIC_WEIGHT_MIN } from './config.constants';

export const configSchema = z.object({
    weights: z.object({
      sameFingerBigrams: z.number().min(METRIC_WEIGHT_MIN).max(METRIC_WEIGHT_MAX),
      fingerDistance: z.number().min(METRIC_WEIGHT_MIN).max(METRIC_WEIGHT_MAX),
      homeRowUsage: z.number().min(METRIC_WEIGHT_MIN).max(METRIC_WEIGHT_MAX),
      handAlternation: z.number().min(METRIC_WEIGHT_MIN).max(METRIC_WEIGHT_MAX),
      rowJumping: z.number().min(METRIC_WEIGHT_MIN).max(METRIC_WEIGHT_MAX),
    }),
    annealing: z.object({
      tStart: z.number().gt(0),
      tMin: z.number().gt(0),
      alpha: z.number().min(0).lt(1),
      iterationsPerTemp: z.number().min(1),
    }),
    seed: z.number().nullable().default(null),
  })
  .refine((config) => config.annealing.tStart > config.annealing.tMin, {
    path: ['annealing', 'tMin'],
    message: 'Minimum temperature must be lower than start temperature',
  });

export type Config = z.infer<typeof configSchema>;
export type MetricsWeights = Config['weights'];
export type AnnealingParams = Config['annealing'];
export type Seed = Config['seed'];

export const defaultConfig: Config = {
  weights: {
    sameFingerBigrams: 1.0,
    fingerDistance: 1.0,
    homeRowUsage: 1.0,
    handAlternation: 1.0,
    rowJumping: 1.0,
  },
  annealing: {
    tStart: 1,
    tMin: 0.001,
    alpha: 0.995,
    iterationsPerTemp: 100,
  },
  seed: 42,
};
