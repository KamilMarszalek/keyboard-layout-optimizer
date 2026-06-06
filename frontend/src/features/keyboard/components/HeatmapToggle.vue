<script setup lang="ts">
import { useCorpusStore } from '@/features/corpus/corpus.store';
import { storeToRefs } from 'pinia';
import { watch } from 'vue';

import { useKeyboardStore } from '../keyboard.store';

const keyboard = useKeyboardStore();
const corpus = useCorpusStore();
const { showHeatmap } = storeToRefs(keyboard);

watch(showHeatmap, (show) => {
  void keyboard.refreshCharFrequencies(show ? corpus.text : '');
});

watch(
  () => corpus.text,
  (text) => {
    if (showHeatmap.value) {
      void keyboard.refreshCharFrequencies(text);
    }
  },
);
</script>

<template>
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
</template>
