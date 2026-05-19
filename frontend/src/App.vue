<script setup lang="ts">
import { storeToRefs } from "pinia";
import { onBeforeUnmount, onMounted } from "vue";
import AppHeader from "./components/layout/AppHeader.vue";
import AnnealingParametersSection from "./components/optimizer/AnnealingParametersSection.vue";
import CorpusSection from "./components/optimizer/CorpusSection.vue";
import MetricWeightsSection from "./components/optimizer/MetricWeightsSection.vue";
import RunSection from "./components/optimizer/RunSection.vue";
import KeyboardPreview from "./components/keyboard/KeyboardPreview.vue";
import CostHistoryList from "./components/results/CostHistoryList.vue";
import MetricsBreakdown from "./components/results/MetricsBreakdown.vue";
import OptimizationSummary from "./components/results/OptimizationSummary.vue";
import { useLayoutStore } from "./stores/layoutStore";
import { useOptimizerStore } from "./stores/optimizerStore";

const store = useOptimizerStore();
const layoutStore = useLayoutStore();
const { result } = storeToRefs(store);

onMounted(() => {
  void layoutStore.loadStandardQwertyLayout();
});

onBeforeUnmount(() => {
  store.dispose();
});
</script>

<template>
  <main class="min-h-screen bg-slate-100 px-4 py-8 text-slate-900 sm:px-6 lg:px-8">
    <div class="mx-auto max-w-7xl space-y-6">
      <AppHeader />

      <div class="grid gap-6 lg:grid-cols-[minmax(0,1.1fr)_minmax(360px,0.9fr)]">
        <CorpusSection />
        <RunSection />
      </div>

      <MetricWeightsSection />
      <AnnealingParametersSection />

      <KeyboardPreview />

      <section
        v-if="result"
        class="grid gap-6 lg:grid-cols-[minmax(0,1fr)_minmax(360px,0.8fr)]"
      >
        <OptimizationSummary />

        <div class="space-y-6">
          <MetricsBreakdown />
          <CostHistoryList />
        </div>
      </section>
    </div>
  </main>
</template>
