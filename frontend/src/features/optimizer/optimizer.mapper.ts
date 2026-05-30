import type { OptimizeRequestDto } from '@/services/optimizer/optimizer.dto';
import type { OptimizeRequest } from './optimizer.schema';

export function toOptimizeRequestDto(values: OptimizeRequest): OptimizeRequestDto {
  return {
    text: values.corpus.text,
    weights: { ...values.config.weights },
    annealing: {
      ...values.config.annealing,
      iterationsPerTemp: Math.max(1, Math.round(values.config.annealing.iterationsPerTemp)),
    },
    seed: values.config.seed ?? undefined,
  };
}
