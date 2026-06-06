import type { OptimizeRequestDto, OptimizeResultDto } from '@/services/optimizer/optimizer.dto';
import type { OptimizeRequest } from './optimizer.schema';
import type { OptimizeResult } from '../results/results.types';

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
    bestLayout: value.bestLayout,
    bestCost: value.bestCost,
    costHistory: value.costHistory,
    metrics: value.metrics,
  };
}
