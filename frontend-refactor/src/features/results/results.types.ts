export interface MetricBreakdown {
  sameFingerBigrams: number;
  fingerDistance: number;
  homeRowUsage: number;
  handAlternation: number;
  rowJumping: number;
}

export interface OptimizeResult {
  bestLayout: string[];
  bestCost: number;
  costHistory: number[];
  metrics: MetricBreakdown;
}
