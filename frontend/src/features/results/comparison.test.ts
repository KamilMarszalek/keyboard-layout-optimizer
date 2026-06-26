import type { EvaluateResult } from '@/features/evaluator/types';
import { describe, expect, it } from 'vitest';

import { toComparisonRows } from './comparison';
import type { MetricBreakdown } from './types';

function makeResult(metrics: MetricBreakdown, totalCost = 0): EvaluateResult {
  return { metrics, totalCost };
}

const userMetrics: MetricBreakdown = {
  sameFingerBigrams: 1,
  fingerDistance: 2,
  homeRowUsage: 3,
  handAlternation: 4,
  rowJumping: 5,
};

const qwertyMetrics: MetricBreakdown = {
  sameFingerBigrams: 6,
  fingerDistance: 7,
  homeRowUsage: 8,
  handAlternation: 9,
  rowJumping: 10,
};

describe('toComparisonRows', () => {
  const rows = toComparisonRows(makeResult(userMetrics), makeResult(qwertyMetrics));

  it('produces one row per metric', () => {
    expect(rows).toHaveLength(5);
  });

  it('pairs the user and qwerty values for each metric', () => {
    expect(rows.map((r) => r.userValue)).toEqual([1, 2, 3, 4, 5]);
    expect(rows.map((r) => r.qwertyValue)).toEqual([6, 7, 8, 9, 10]);
  });

  it('marks the correct direction of "better" per metric', () => {
    const byLabel = Object.fromEntries(rows.map((r) => [r.label, r.lowerIsBetter]));
    expect(byLabel['Same finger bigrams']).toBe(true);
    expect(byLabel['Finger distance']).toBe(true);
    expect(byLabel['Row jumping']).toBe(true);
    expect(byLabel['Home row usage']).toBe(false);
    expect(byLabel['Hand alternation']).toBe(false);
  });

  it('attaches a human-readable label to every row', () => {
    expect(rows.every((r) => r.label.length > 0)).toBe(true);
  });
});
