import type { EvaluateResult } from '@/features/evaluator/types';
import type { MetricBreakdown } from '@/features/results/types';

import { metricLabels } from './controls';

export interface ComparisonRow {
  label: string;
  userValue: number;
  qwertyValue: number;
  lowerIsBetter: boolean;
}

const lowerIsBetter: Record<keyof MetricBreakdown, boolean> = {
  sameFingerBigrams: true,
  fingerDistance: true,
  homeRowUsage: false,
  handAlternation: false,
  rowJumping: true,
};

export function toComparisonRows(user: EvaluateResult, qwerty: EvaluateResult): ComparisonRow[] {
  return metricLabels.map((metric) => ({
    label: metric.label,
    userValue: user.metrics[metric.key],
    qwertyValue: qwerty.metrics[metric.key],
    lowerIsBetter: lowerIsBetter[metric.key],
  }));
}
