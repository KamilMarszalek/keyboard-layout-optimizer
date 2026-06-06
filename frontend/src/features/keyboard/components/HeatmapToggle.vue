<script setup lang="ts">
import { useCorpusStore } from '@/features/corpus/store';
import { storeToRefs } from 'pinia';
import { watchEffect } from 'vue';

import { useKeyboardStore } from '../store';

const keyboard = useKeyboardStore();
const corpus = useCorpusStore();
const { showHeatmap } = storeToRefs(keyboard);

watchEffect(() => {
  if (showHeatmap.value) {
    keyboard.refreshCharFrequencies(corpus.text);
  }
});
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
