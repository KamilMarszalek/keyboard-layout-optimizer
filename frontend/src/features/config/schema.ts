import { z } from 'zod';

import { METRIC_WEIGHT_MAX, METRIC_WEIGHT_MIN } from './constants';

const weightSchema = z.number().min(METRIC_WEIGHT_MIN).max(METRIC_WEIGHT_MAX);

export const weightsSchema = z.object({
  sameFingerBigrams: weightSchema,
  fingerDistance: weightSchema,
  homeRowUsage: weightSchema,
  handAlternation: weightSchema,
  rowJumping: weightSchema,
});

export const annealingSchema = z
  .object({
    tStart: z.number().gt(0),
    tMin: z.number().gt(0),
    alpha: z.number().min(0).lt(1),
    iterationsPerTemp: z.number().min(1).int(),
  })
  .refine((schema) => schema.tStart > schema.tMin, {
    path: ['annealing', 'tMin'],
    message: 'Minimum temperature must be lower than start temperature',
  });

export const configSchema = z.object({
  weights: weightsSchema,
  annealing: annealingSchema,
  seed: z.number().nullable().default(null),
});

export type MetricsWeights = z.infer<typeof weightsSchema>;
export type AnnealingParams = z.infer<typeof annealingSchema>;
export type Config = z.infer<typeof configSchema>;

export const defaultWeights: MetricsWeights = {
  sameFingerBigrams: 1.0,
  fingerDistance: 1.0,
  homeRowUsage: 1.0,
  handAlternation: 1.0,
  rowJumping: 1.0,
};

export const defaultAnnealingParams: AnnealingParams = {
  tStart: 1,
  tMin: 0.001,
  alpha: 0.995,
  iterationsPerTemp: 100,
};

export const defaultSeed: number = 42;
