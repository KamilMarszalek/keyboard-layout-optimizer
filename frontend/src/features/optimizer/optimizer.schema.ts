import { configSchema } from '@/features/config/config.schema';
import { corpusSchema } from '@/features/corpus/corpus.schema';
import { z } from 'zod';

export const optimizeRequestSchema = z.object({
  ...configSchema.shape,
  ...corpusSchema.shape,
});

export type OptimizeRequest = z.infer<typeof optimizeRequestSchema>;
