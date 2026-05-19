import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { metricControls } from "../constants/optimizerControls";
import { formatError } from "../lib/format";
import { disposeOptimizerWorker, optimizeInWorker } from "../services/optimizerWorkerClient";
import type { AnnealingConfigDto, MetricWeightsDto, OptimizeRequestDto, OptimizeResultDto } from "../wasmTypes";

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

export const useOptimizerStore = defineStore("optimizer", () => {
  const corpusText = ref(sampleText);
  const metricWeights = ref<MetricWeightsDto>(defaultWeights());
  const annealing = ref<AnnealingConfigDto>(defaultAnnealing());
  const seed = ref<number | "">(42);
  const result = ref<OptimizeResultDto | null>(null);
  const validationError = ref<string | null>(null);
  const runtimeError = ref<string | null>(null);
  const isOptimizing = ref(false);

  const displayedError = computed(() => validationError.value ?? runtimeError.value);
  const corpusCharacterCount = computed(() => corpusText.value.trim().length);
  const recentCostHistory = computed(() => result.value?.costHistory.slice(-8) ?? []);
  const resultMetrics = computed(() =>
    metricControls.map(({ key, label }) => ({
      key,
      label,
      value: result.value?.metrics[key] ?? 0,
    })),
  );
  const costHistoryLength = computed(() => result.value?.costHistory.length ?? 0);
  const bestCost = computed(() => result.value?.bestCost ?? null);

  function buildRequest(): OptimizeRequestDto | null {
    validationError.value = null;
    const trimmedText = corpusText.value.trim();

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
      weights: { ...metricWeights.value },
      annealing: {
        ...annealing.value,
        iterationsPerTemp: Math.max(1, Math.round(annealing.value.iterationsPerTemp)),
      },
      seed: seedValue,
    };
  }

  async function runOptimization() {
    if (isOptimizing.value) {
      return;
    }

    const request = buildRequest();
    if (!request) {
      return;
    }

    isOptimizing.value = true;
    runtimeError.value = null;

    try {
      result.value = await optimizeInWorker(request);
    } catch (caught) {
      runtimeError.value = formatError(caught);
    } finally {
      isOptimizing.value = false;
    }
  }

  function dispose() {
    disposeOptimizerWorker();
  }

  return {
    annealing,
    bestCost,
    corpusCharacterCount,
    corpusText,
    costHistoryLength,
    displayedError,
    dispose,
    isOptimizing,
    metricWeights,
    recentCostHistory,
    result,
    resultMetrics,
    runOptimization,
    runtimeError,
    seed,
    validationError,
  };
});
