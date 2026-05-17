<script setup lang="ts">
import { computed } from "vue";
import { metricControls } from "../constants/optimizerControls";
import type { MetricBreakdownDto } from "../wasmTypes";

const props = defineProps<{
  metrics: MetricBreakdownDto;
}>();

const resultMetrics = computed(() =>
  metricControls.map(({ key, label }) => ({
    key,
    label,
    value: props.metrics[key] ?? 0,
  })),
);

function formatNumber(value: number): string {
  return value.toLocaleString(undefined, {
    maximumFractionDigits: 6,
  });
}
</script>

<template>
  <div class="rounded-xl bg-white p-5 shadow-sm ring-1 ring-slate-200 sm:p-6">
    <h2 class="text-xl font-semibold text-slate-950">
      Metrics breakdown
    </h2>
    <dl class="mt-4 space-y-3">
      <div
        v-for="metric in resultMetrics"
        :key="metric.key"
        class="flex items-center justify-between gap-4 rounded-lg bg-slate-50 px-4 py-3 ring-1 ring-slate-200"
      >
        <dt class="text-sm text-slate-600">
          {{ metric.label }}
        </dt>
        <dd class="text-sm font-semibold text-slate-950">
          {{ formatNumber(metric.value) }}
        </dd>
      </div>
    </dl>
  </div>
</template>
