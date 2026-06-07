<script setup lang="ts">
import { storeToRefs } from 'pinia';

import { useOptimizerResultStore } from '../store';

const store = useOptimizerResultStore();
const { result, costHistory } = storeToRefs(store);
</script>

<template>
  <div>
    <p class="text-sm font-medium">Optimization result</p>
    <div class="mt-3 grid grid-cols-2 gap-4">
      <div class="text-center">
        <p class="text-xs text-muted-foreground">Best cost</p>
        <p class="mt-1 text-2xl font-bold">
          {{ result?.bestCost.toFixed(4) ?? '-' }}
        </p>
      </div>
      <div class="text-center">
        <p class="text-xs text-muted-foreground">Cost history length</p>
        <p class="mt-1 text-2xl font-bold">{{ costHistory.length }}</p>
      </div>
    </div>
  </div>
</template>
