export interface MetricWeightsDto {
  sameFingerBigrams: number;
  fingerDistance: number;
  homeRowUsage: number;
  handAlternation: number;
  rowJumping: number;
}

export interface AnnealingConfigDto {
  tStart: number;
  tMin: number;
  alpha: number;
  iterationsPerTemp: number;
}

export interface OptimizeRequestDto {
  text: string;
  weights: MetricWeightsDto;
  annealing: AnnealingConfigDto;
  seed: number | null;
}

export interface MetricBreakdownDto {
  sameFingerBigrams: number;
  fingerDistance: number;
  homeRowUsage: number;
  handAlternation: number;
  rowJumping: number;
}

export interface KeyMappingDto {
  base: string;
  shifted: string;
}

export interface LayoutDto {
  mappings: KeyMappingDto[];
}

export interface OptimizeResultDto {
  bestLayout: LayoutDto;
  bestCost: number;
  costHistory: number[];
  metrics: MetricBreakdownDto;
}

export interface CharFrequencyDto {
  key: string;
  frequency: number;
}
