<script setup lang="ts">
import { metricControls } from "../constants/optimizerControls";
import type { MetricWeightsDto } from "../wasmTypes";

const weights = defineModel<MetricWeightsDto>("weights", { required: true });
</script>

<template>
  <section class="rounded-xl bg-white p-5 shadow-sm ring-1 ring-slate-200 sm:p-6">
    <div class="flex flex-col gap-1 sm:flex-row sm:items-end sm:justify-between">
      <div>
        <h2 class="text-xl font-semibold text-slate-950">
          Metric weights
        </h2>
        <p class="mt-1 text-sm text-slate-600">
          Set each objective from 0.0 to 5.0.
        </p>
      </div>
    </div>

    <div class="mt-5 grid gap-4 md:grid-cols-2">
      <div
        v-for="metric in metricControls"
        :key="metric.key"
        class="rounded-lg border border-slate-200 bg-slate-50 p-4"
      >
        <div class="flex items-start justify-between gap-4">
          <div>
            <label :for="`weight-${metric.key}`" class="text-sm font-medium text-slate-800">
              {{ metric.label }}
            </label>
            <p class="mt-1 text-xs leading-5 text-slate-500">
              {{ metric.help }}
            </p>
          </div>
          <span class="rounded-md bg-white px-2 py-1 text-sm font-semibold text-slate-800 ring-1 ring-slate-200">
            {{ weights[metric.key].toFixed(1) }}
          </span>
        </div>
        <input
          :id="`weight-${metric.key}`"
          v-model.number="weights[metric.key]"
          type="range"
          min="0"
          max="5"
          step="0.1"
          class="mt-4 h-2 w-full cursor-pointer accent-teal-700"
        />
      </div>
    </div>
  </section>
</template>
