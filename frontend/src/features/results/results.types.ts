export interface MetricBreakdown {
  sameFingerBigrams: number;
  fingerDistance: number;
  homeRowUsage: number;
  handAlternation: number;
  rowJumping: number;
}

export interface KeyMapping {
  base: string;
  shifted: string;
}

export interface Layout {
  mappings: KeyMapping[];
}

export interface OptimizeResult {
  bestLayout: Layout;
  bestCost: number;
  costHistory: number[];
  metrics: MetricBreakdown;
}
