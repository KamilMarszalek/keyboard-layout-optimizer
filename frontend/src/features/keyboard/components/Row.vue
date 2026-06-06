<script setup lang="ts">
import { keyHeatStyle } from '../heatmap';
import type { KeyMapping } from '../types';
import Key from './Key.vue';

const props = defineProps<{
  mappings: KeyMapping[];
  offsetClass?: string;
  freqMap: Map<string, number>;
  maxFreq: number;
}>();
</script>

<template>
  <div class="flex gap-2" :class="offsetClass">
    <Key
      v-for="(mapping, keyIndex) in mappings"
      :key="`${keyIndex}-${mapping.base}`"
      :base="mapping.base"
      :shifted="mapping.shifted"
      :heat-style="keyHeatStyle(mapping.base, props.freqMap, props.maxFreq)"
    />
  </div>
</template>
