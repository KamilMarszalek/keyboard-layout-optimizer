import { z } from 'zod';
import { configSchema } from '@/features/config/config.schema';
import { corpusSchema } from '@/features/corpus/corpus.schema';

export const optimizeRequestSchema = z.object({
  config: configSchema,
  corpus: corpusSchema,
});

export type OptimizeRequest = z.infer<typeof optimizeRequestSchema>;
