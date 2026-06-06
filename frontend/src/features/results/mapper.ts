import type { MetricBreakdownDto, OptimizeResultDto } from '@/wasm/dto';

import { fromLayoutDto } from '../keyboard/mapper';
import type { MetricBreakdown, OptimizeResult } from './results.types';

export function fromMetricBreakdownDto(value: MetricBreakdownDto): MetricBreakdown {
  return {
    sameFingerBigrams: value.sameFingerBigrams,
    fingerDistance: value.fingerDistance,
    homeRowUsage: value.homeRowUsage,
    handAlternation: value.handAlternation,
    rowJumping: value.rowJumping,
  };
}

export function fromOptimizeResultDto(value: OptimizeResultDto): OptimizeResult {
  return {
    bestLayout: fromLayoutDto(value.bestLayout),
    metrics: fromMetricBreakdownDto(value.metrics),
    bestCost: value.bestCost,
    costHistory: value.costHistory,
  };
}
