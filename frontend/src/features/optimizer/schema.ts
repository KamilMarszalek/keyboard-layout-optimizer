import { configSchema } from '@/features/config/schema';
import { corpusSchema } from '@/features/corpus/schema';
import { z } from 'zod';

export const optimizeRequestSchema = z.object({
  ...configSchema.shape,
  ...corpusSchema.shape,
});

export type OptimizeRequest = z.infer<typeof optimizeRequestSchema>;
