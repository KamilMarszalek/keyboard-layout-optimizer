import type { Layout } from '@/features/keyboard/types';

export interface MetricBreakdown {
  sameFingerBigrams: number;
  fingerDistance: number;
  homeRowUsage: number;
  handAlternation: number;
  rowJumping: number;
}

export interface OptimizeResult {
  bestLayout: Layout;
  bestCost: number;
  costHistory: number[];
  metrics: MetricBreakdown;
}
