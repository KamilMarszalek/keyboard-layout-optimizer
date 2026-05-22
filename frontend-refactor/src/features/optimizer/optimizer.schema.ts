import { z } from 'zod';
import { annealingParamsSchema, metricWeightsSchema } from '@/features/config/config.schema';

export const optimizeRequestSchema = z.object({
  weights: metricWeightsSchema,
  annealing: annealingParamsSchema,
  text: z.string().min(1),
  seed: z.number(),
});

export type OptimizeRequest = z.infer<typeof optimizeRequestSchema>;
