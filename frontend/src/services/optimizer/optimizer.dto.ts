import { z } from 'zod';

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

const metricBreakdownSchema = z.object({
  sameFingerBigrams: z.number(),
  fingerDistance: z.number(),
  homeRowUsage: z.number(),
  handAlternation: z.number(),
  rowJumping: z.number(),
});

export const optimizeResultSchema = z.object({
  bestLayout: z.array(z.string()),
  bestCost: z.number(),
  costHistory: z.array(z.number()),
  metrics: metricBreakdownSchema,
});

export interface CharFrequencyDto {
  key: string;
  frequency: number;
}

const charFrequencyDtoSchema = z.object({
  key: z.string(),
  frequency: z.number(),
});

export const charFrequenciesSchema = z.array(charFrequencyDtoSchema);
