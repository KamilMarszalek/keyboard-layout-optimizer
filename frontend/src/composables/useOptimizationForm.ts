import { ref } from "vue";
import type { AnnealingConfigDto, MetricWeightsDto, OptimizeRequestDto } from "../wasmTypes";

const sampleText =
  "The quick brown fox jumps over the lazy dog while the keyboard optimizer searches for a smoother typing layout.";

function defaultWeights(): MetricWeightsDto {
  return {
    sameFingerBigrams: 1.0,
    fingerDistance: 1.0,
    homeRowUsage: 1.0,
    handAlternation: 1.0,
    rowJumping: 1.0,
  };
}

function defaultAnnealing(): AnnealingConfigDto {
  return {
    tStart: 1.0,
    tMin: 0.0001,
    alpha: 0.995,
    iterationsPerTemp: 100,
  };
}

export function useOptimizationForm() {
  const text = ref(sampleText);
  const seed = ref<number | "">(42);
  const weights = ref<MetricWeightsDto>(defaultWeights());
  const annealing = ref<AnnealingConfigDto>(defaultAnnealing());
  const validationError = ref<string | null>(null);

  function buildRequest(): OptimizeRequestDto | null {
    validationError.value = null;
    const trimmedText = text.value.trim();

    if (!trimmedText) {
      validationError.value = "Enter some corpus text before starting optimization.";
      return null;
    }

    const seedValue = seed.value === "" ? undefined : Number(seed.value);
    if (seedValue !== undefined && !Number.isFinite(seedValue)) {
      validationError.value = "Seed must be a valid number, or left empty.";
      return null;
    }

    return {
      text: trimmedText,
      weights: { ...weights.value },
      annealing: {
        ...annealing.value,
        iterationsPerTemp: Math.max(1, Math.round(annealing.value.iterationsPerTemp)),
      },
      seed: seedValue,
    };
  }

  return {
    text,
    weights,
    annealing,
    seed,
    validationError,
    buildRequest,
  };
}
