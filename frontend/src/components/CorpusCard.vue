<script setup lang="ts">
import { computed } from "vue";

const text = defineModel<string>({ required: true });
const characterCount = computed(() => text.value.trim().length);
</script>

<template>
  <section class="rounded-xl bg-white p-5 shadow-sm ring-1 ring-slate-200 sm:p-6">
    <div class="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
      <div>
        <h2 class="text-xl font-semibold text-slate-950">
          Corpus
        </h2>
        <p class="mt-1 text-sm text-slate-600">
          Paste representative text for the optimizer to evaluate.
        </p>
      </div>
      <span class="rounded-full bg-slate-100 px-3 py-1 text-sm text-slate-600">
        {{ characterCount }} characters
      </span>
    </div>

    <label for="corpus-text" class="mt-5 block text-sm font-medium text-slate-700">
      Input text
    </label>
    <textarea
      id="corpus-text"
      v-model="text"
      rows="9"
      class="mt-2 block w-full resize-y rounded-lg border border-slate-300 bg-white px-3 py-3 text-sm leading-6 text-slate-900 shadow-sm outline-none transition focus:border-teal-500 focus:ring-4 focus:ring-teal-100"
    ></textarea>
  </section>
</template>
