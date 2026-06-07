import type { MetricBreakdown } from '@/features/results/types';

export interface EvaluateResult {
  metrics: MetricBreakdown;
  totalCost: number;
}
