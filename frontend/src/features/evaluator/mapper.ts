import type { MetricsWeights } from '@/features/config/schema';
import { fromMetricBreakdownDto } from '@/features/results/mapper';
import type { EvaluateRequestDto, EvaluateResultDto } from '@/wasm/dto';

import type { EvaluateResult } from './types';

export function fromEvaluateResultDto(dto: EvaluateResultDto): EvaluateResult {
  return {
    metrics: fromMetricBreakdownDto(dto.metrics),
    totalCost: dto.totalCost,
  };
}

export function toEvaluateRequestDto(
  keys: string[],
  text: string,
  weights: MetricsWeights,
): EvaluateRequestDto {
  return {
    keys,
    text,
    weights,
  };
}
