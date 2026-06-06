import type { OptimizeRequestDto } from '@/wasm/dto';

import type { OptimizeRequest } from './schema';

export function toOptimizeRequestDto(value: OptimizeRequest): OptimizeRequestDto {
  return {
    weights: value.weights,
    annealing: value.annealing,
    seed: value.seed,
    text: value.text,
  };
}
