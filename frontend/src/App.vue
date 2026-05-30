<script setup lang="ts">
import Header from '@/components/common/Header.vue';
import { OptimizerForm } from '@/features/optimizer/components';
import { useOptimizerStore } from '@/features/optimizer/optimizer.store';
import { KeyboardPreview } from '@/features/keyboard/components';
import { useKeyboardStore } from '@/features/keyboard/keyboard.store';
import { useResultsStore } from '@/features/results/results.store';
import { storeToRefs } from 'pinia';
import { onBeforeUnmount, onMounted } from 'vue';
import { CostHistory, MetricsBreakdown, OptimizationResult } from '@/features/results/components';

const optimizerStore = useOptimizerStore();
const keyboardStore = useKeyboardStore();
const resultsStore = useResultsStore();
const { result } = storeToRefs(resultsStore);

onMounted(() => {
  void keyboardStore.loadStandardQwertyLayout();
});

onBeforeUnmount(() => {
  optimizerStore.dispose();
});
</script>

<template>
  <main class="min-h-screen px-4 py-8 sm:px-6 lg:px-8">
    <div class="mx-auto max-w-7xl space-y-6">
      <Header />
      <OptimizerForm />
      <KeyboardPreview :optimized-layout="result?.bestLayout" />

      <section
        v-if="result"
        class="grid gap-6 lg:grid-cols-[minmax(0,1fr)_minmax(360px,0.8fr)]"
      >
        <OptimizationResult />
        <div class="space-y-6">
          <MetricsBreakdown />
          <CostHistory />
        </div>
      </section>
    </div>
  </main>
</template>
