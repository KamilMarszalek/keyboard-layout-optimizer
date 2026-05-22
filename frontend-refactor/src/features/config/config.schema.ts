import { z } from 'zod';

export const configSchema = z.object({
  weights: z.object({
    sameFingerBigrams: z.number().min(0).max(5),
    fingerDistance: z.number().min(0).max(5),
    homeRowUsage: z.number().min(0).max(5),
    handAlternation: z.number().min(0).max(5),
    rowJumping: z.number().min(0).max(5),
  }),
  annealing: z.object({
    tStart: z.number().min(0),
    tMin: z.number().min(0),
    alpha: z.number().min(0).max(1),
    iterationsPerTemp: z.number().min(0),
  }),
  seed: z.number().nullable().default(null),
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
