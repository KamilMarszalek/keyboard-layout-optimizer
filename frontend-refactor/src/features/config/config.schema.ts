import { z } from 'zod';

export const metricWeightsSchema = z.object({
  sameFingerBigrams: z.number().min(0).max(5).default(1.0),
  fingerDistance: z.number().min(0).max(5).default(1.0),
  homeRowUsage: z.number().min(0).max(5).default(1.0),
  handAlternation: z.number().min(0).max(5).default(1.0),
  rowJumping: z.number().min(0).max(5).default(1.0),
});

export type MetricsWeights = z.infer<typeof metricWeightsSchema>;
export const defaultMetricWeithgs = metricWeightsSchema.parse({});

export const annealingParamsSchema = z.object({
  tStart: z.number().min(0).default(1),
  tMin: z.number().min(0).default(0.0001),
  alpha: z.number().min(0).max(1).default(0.995),
  iterationsPerTemp: z.number().min(0).default(100),
});

export type AnnealingParams = z.infer<typeof annealingParamsSchema>;
export const defaultAnnealingParams = annealingParamsSchema.parse({});
