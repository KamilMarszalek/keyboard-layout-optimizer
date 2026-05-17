<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import AnnealingParametersCard from "./components/AnnealingParametersCard.vue";
import CorpusCard from "./components/CorpusCard.vue";
import CostHistoryList from "./components/CostHistoryList.vue";
import KeyboardPreview from "./components/KeyboardPreview.vue";
import MetricWeightsCard from "./components/MetricWeightsCard.vue";
import MetricsBreakdown from "./components/MetricsBreakdown.vue";
import OptimizationSummary from "./components/OptimizationSummary.vue";
import RunCard from "./components/RunCard.vue";
import { useKeyboardLayout } from "./composables/useKeyboardLayout";
import { useOptimizationForm } from "./composables/useOptimizationForm";
import { useOptimizerWorker } from "./composables/useOptimizerWorker";
import type { OptimizeResultDto } from "./wasmTypes";

const result = ref<OptimizeResultDto | null>(null);
const runError = ref<string | null>(null);
const isOptimizing = ref(false);

const { text, weights, annealing, seed, validationError, buildRequest } = useOptimizationForm();
const { optimizeInWorker, disposeWorker } = useOptimizerWorker();
const {
  currentLayout,
  layoutTitle,
  layoutValidationMessage,
  expectedLayoutLength,
  loadStandardQwertyLayout,
} = useKeyboardLayout(result);

const displayedError = computed(() => validationError.value ?? runError.value);

function formatError(caught: unknown): string {
  if (caught instanceof Error) {
    return caught.message;
  }

  return String(caught);
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
  runError.value = null;

  try {
    result.value = await optimizeInWorker(request);
  } catch (caught) {
    runError.value = formatError(caught);
  } finally {
    isOptimizing.value = false;
  }
}

onBeforeUnmount(() => {
  disposeWorker();
});

onMounted(() => {
  void loadStandardQwertyLayout();
});
</script>

<template>
  <main class="min-h-screen bg-slate-100 px-4 py-8 text-slate-900 sm:px-6 lg:px-8">
    <div class="mx-auto max-w-7xl space-y-6">
      <header class="space-y-2">
        <p class="text-sm font-semibold uppercase tracking-wide text-teal-700">
          Keyboard layout optimizer
        </p>
        <h1 class="text-3xl font-semibold tracking-tight text-slate-950 sm:text-4xl">
          Optimize a layout for your own corpus
        </h1>
        <p class="max-w-3xl text-base leading-7 text-slate-600">
          Tune metric weights and simulated annealing parameters, then run the
          Rust/WASM optimizer directly in the browser.
        </p>
      </header>

      <div class="grid gap-6 lg:grid-cols-[minmax(0,1.1fr)_minmax(360px,0.9fr)]">
        <CorpusCard v-model="text" />
        <RunCard
          v-model:seed="seed"
          :error="displayedError"
          :is-optimizing="isOptimizing"
          @optimize="runOptimization"
        />
      </div>

      <MetricWeightsCard v-model:weights="weights" />
      <AnnealingParametersCard v-model:annealing="annealing" />

      <KeyboardPreview
        :title="layoutTitle"
        :layout="currentLayout"
        :expected-layout-length="expectedLayoutLength"
        :validation-message="layoutValidationMessage"
      />

      <section
        v-if="result"
        class="grid gap-6 lg:grid-cols-[minmax(0,1fr)_minmax(360px,0.8fr)]"
      >
        <OptimizationSummary
          :best-cost="result.bestCost"
          :cost-history-length="result.costHistory.length"
        />

        <div class="space-y-6">
          <MetricsBreakdown :metrics="result.metrics" />
          <CostHistoryList :cost-history="result.costHistory" />
        </div>
      </section>
    </div>
  </main>
</template>
