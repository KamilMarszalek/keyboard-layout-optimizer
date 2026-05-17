import type { AnnealingConfigDto, MetricWeightsDto } from "../wasmTypes";

export const metricControls = [
  {
    key: "sameFingerBigrams",
    label: "Same finger bigrams",
    help: "Penalty for repeated use of the same finger on adjacent letters.",
  },
  {
    key: "fingerDistance",
    label: "Finger distance",
    help: "Penalty for longer finger travel across the keyboard.",
  },
  {
    key: "homeRowUsage",
    label: "Home row usage",
    help: "Reward for keeping common keys near the home row.",
  },
  {
    key: "handAlternation",
    label: "Hand alternation",
    help: "Reward for alternating between hands while typing.",
  },
  {
    key: "rowJumping",
    label: "Row jumping",
    help: "Penalty for movement between distant keyboard rows.",
  },
] as const satisfies ReadonlyArray<{
  key: keyof MetricWeightsDto;
  label: string;
  help: string;
}>;

type AnnealingField = {
  key: keyof AnnealingConfigDto;
  label: string;
  step: string;
  min: string;
  max?: string;
};

export const annealingFields = [
  {
    key: "tStart",
    label: "Start temperature",
    step: "0.1",
    min: "0",
  },
  {
    key: "tMin",
    label: "Minimum temperature",
    step: "0.0001",
    min: "0",
  },
  {
    key: "alpha",
    label: "Cooling alpha",
    step: "0.0001",
    min: "0",
    max: "1",
  },
  {
    key: "iterationsPerTemp",
    label: "Iterations per temperature",
    step: "1",
    min: "1",
  },
] satisfies AnnealingField[];
