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
    seed?: number;
}

export interface MetricBreakdownDto {
    sameFingerBigrams: number;
    fingerDistance: number;
    homeRowUsage: number;
    handAlternation: number;
    rowJumping: number;
}

export interface OptimizeResultDto {
  bestLayout: string[];
  bestCost: number;
  costHistory: number[];
  metrics: MetricBreakdownDto;
}