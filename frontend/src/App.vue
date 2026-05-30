<script setup lang="ts">
import Header from '@/components/common/Header.vue';
import { OptimizerForm } from '@/features/optimizer/components';
import { useOptimizerStore } from '@/features/optimizer/optimizer.store';
import { KeyboardPreview } from '@/features/keyboard/components';
import { useKeyboardStore } from '@/features/keyboard/keyboard.store';
import { useResultsStore } from '@/features/results/results.store';
import type { CharFrequencyDto } from '@/services/optimizer/optimizer.dto';
import { getCharFrequencies } from '@/services/optimizer/wasmClient';
import { storeToRefs } from 'pinia';
import { onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { CostHistory, MetricsBreakdown, OptimizationResult } from '@/features/results/components';

const optimizerStore = useOptimizerStore();
const keyboardStore = useKeyboardStore();
const resultsStore = useResultsStore();
const { result } = storeToRefs(resultsStore);

const showHeatmap = ref(false);
const charFrequencies = ref<CharFrequencyDto[] | undefined>(undefined);

async function refreshCharFrequencies(text: string) {
  charFrequencies.value = text.trim()
    ? await getCharFrequencies(text)
    : undefined;
}

watch(showHeatmap, async (show) => {
  if (show) {
    await refreshCharFrequencies(keyboardStore.corpusText);
  } else {
    charFrequencies.value = undefined;
  }
});

watch(() => keyboardStore.corpusText, async (text) => {
  if (showHeatmap.value) {
    await refreshCharFrequencies(text);
  }
});

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
      <div class="flex items-center gap-2">
        <input
          id="show-heatmap"
          v-model="showHeatmap"
          type="checkbox"
          class="h-4 w-4 cursor-pointer rounded border-border accent-amber-500"
        />
        <label for="show-heatmap" class="cursor-pointer select-none text-sm text-muted-foreground">
          Show heat map
        </label>
      </div>
      <KeyboardPreview
        :optimized-layout="result?.bestLayout"
        :char-frequencies="charFrequencies"
      />

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
