<script setup lang="ts">
import type { KeyView } from '../heatmap';

defineProps<{
  view: KeyView;
}>();
</script>

<template>
  <div
    class="relative flex h-12 w-12 shrink-0 items-center justify-center rounded-md border border-border bg-card font-mono text-base font-semibold text-card-foreground shadow-sm"
    :style="view.style"
  >
    <span
      v-if="view.mapping.shifted"
      class="absolute right-1 top-1 text-[0.625rem] font-medium leading-none text-muted-foreground"
    >
      {{ view.mapping.shifted }}
    </span>
    {{ view.mapping.base }}
  </div>
</template>
