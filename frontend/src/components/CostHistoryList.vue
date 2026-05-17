<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  costHistory: number[];
}>();

const recentCostHistory = computed(() => props.costHistory.slice(-8));

function formatNumber(value: number): string {
  return value.toLocaleString(undefined, {
    maximumFractionDigits: 6,
  });
}
</script>

<template>
  <div class="rounded-xl bg-white p-5 shadow-sm ring-1 ring-slate-200 sm:p-6">
    <h2 class="text-xl font-semibold text-slate-950">
      Recent cost history
    </h2>
    <ol class="mt-4 space-y-2">
      <li
        v-for="(value, index) in recentCostHistory"
        :key="`${index}-${value}`"
        class="flex items-center justify-between gap-4 rounded-lg bg-slate-50 px-4 py-2 text-sm ring-1 ring-slate-200"
      >
        <span class="text-slate-500">
          #{{ costHistory.length - recentCostHistory.length + index + 1 }}
        </span>
        <span class="font-medium text-slate-950">{{ formatNumber(value) }}</span>
      </li>
    </ol>
  </div>
</template>
