import { fromLayoutDto } from '@/features/keyboard/mapper';
import type { OptimizeRequestDto, OptimizeResultDto } from '@/wasm/dto';

import type { OptimizeResult } from '../results/results.types';
import type { OptimizeRequest } from './optimizer.schema';

export function toOptimizeRequestDto(value: OptimizeRequest): OptimizeRequestDto {
  return {
    weights: value.weights,
    annealing: value.annealing,
    seed: value.seed,
    text: value.text,
  };
}

export function fromOptimizeResultDto(value: OptimizeResultDto): OptimizeResult {
  return {
    bestLayout: fromLayoutDto(value.bestLayout),
    bestCost: value.bestCost,
    costHistory: value.costHistory,
    metrics: value.metrics,
  };
}
