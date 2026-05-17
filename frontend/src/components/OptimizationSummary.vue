<script setup lang="ts">
defineProps<{
  bestCost: number;
  costHistoryLength: number;
}>();

function formatNumber(value: number): string {
  return value.toLocaleString(undefined, {
    maximumFractionDigits: 6,
  });
}
</script>

<template>
  <div class="rounded-xl bg-white p-5 shadow-sm ring-1 ring-slate-200 sm:p-6">
    <h2 class="text-xl font-semibold text-slate-950">
      Optimization result
    </h2>
    <div class="mt-5 grid gap-4 sm:grid-cols-2">
      <div class="rounded-lg bg-slate-50 p-4 ring-1 ring-slate-200">
        <p class="text-sm text-slate-600">
          Best cost
        </p>
        <p class="mt-2 text-2xl font-semibold text-slate-950">
          {{ formatNumber(bestCost) }}
        </p>
      </div>
      <div class="rounded-lg bg-slate-50 p-4 ring-1 ring-slate-200">
        <p class="text-sm text-slate-600">
          Cost history length
        </p>
        <p class="mt-2 text-2xl font-semibold text-slate-950">
          {{ costHistoryLength }}
        </p>
      </div>
    </div>
  </div>
</template>
