<script setup lang="ts">
import type { KeyHeatStyle } from '../heatmap';

defineProps<{
  base: string;
  shifted: string;
  heatStyle?: KeyHeatStyle;
}>();
</script>

<template>
  <div
    class="relative flex h-12 w-12 shrink-0 items-center justify-center rounded-md border border-border bg-card font-mono text-base font-semibold text-card-foreground shadow-sm"
    :style="heatStyle"
  >
    <span
      v-if="shifted"
      class="absolute right-1 top-1 text-[0.625rem] font-medium leading-none text-muted-foreground"
    >
      {{ shifted }}
    </span>
    {{ base }}
  </div>
</template>
