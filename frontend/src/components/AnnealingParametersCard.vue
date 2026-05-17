<script setup lang="ts">
import { annealingFields } from "../constants/optimizerControls";
import type { AnnealingConfigDto } from "../wasmTypes";

const annealing = defineModel<AnnealingConfigDto>("annealing", { required: true });
</script>

<template>
  <section class="rounded-xl bg-white p-5 shadow-sm ring-1 ring-slate-200 sm:p-6">
    <h2 class="text-xl font-semibold text-slate-950">
      Annealing parameters
    </h2>
    <p class="mt-1 text-sm text-slate-600">
      Control the cooling schedule and work performed at each temperature.
      Higher iteration counts can take longer in the browser.
    </p>

    <div class="mt-5 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
      <div v-for="field in annealingFields" :key="field.key">
        <label :for="`annealing-${field.key}`" class="block text-sm font-medium text-slate-700">
          {{ field.label }}
        </label>
        <input
          :id="`annealing-${field.key}`"
          v-model.number="annealing[field.key]"
          type="number"
          :step="field.step"
          :min="field.min"
          :max="field.max"
          class="mt-2 block w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 shadow-sm outline-none transition focus:border-teal-500 focus:ring-4 focus:ring-teal-100"
        />
      </div>
    </div>
  </section>
</template>
