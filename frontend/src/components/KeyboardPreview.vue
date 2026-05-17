<script setup lang="ts">
import { computed } from "vue";
import { KEYBOARD_ROW_OFFSETS, layoutToRows } from "../keyboard";

const props = defineProps<{
  title: string;
  layout: string[];
  expectedLayoutLength: number;
  validationMessage?: string | null;
}>();

const displayedLayoutRows = computed(() => layoutToRows(props.layout));
</script>

<template>
  <section class="rounded-xl bg-white p-5 shadow-sm ring-1 ring-slate-200 sm:p-6">
    <div class="flex flex-col gap-1 sm:flex-row sm:items-end sm:justify-between">
      <div>
        <h2 class="text-xl font-semibold text-slate-950">
          {{ title }}
        </h2>
        <p class="mt-1 text-sm text-slate-600">
          Keys are shown in fixed physical ANSI positions.
        </p>
      </div>
      <span class="rounded-full bg-slate-100 px-3 py-1 text-sm text-slate-600">
        {{ expectedLayoutLength }} keys
      </span>
    </div>

    <p
      v-if="validationMessage"
      class="mt-4 rounded-lg border border-amber-200 bg-amber-50 px-4 py-3 text-sm text-amber-800"
    >
      {{ validationMessage }}
    </p>

    <div class="mt-5 overflow-x-auto pb-2">
      <div class="mx-auto w-max space-y-2 rounded-lg bg-slate-50 p-4 ring-1 ring-slate-200">
        <div
          v-for="(row, rowIndex) in displayedLayoutRows"
          :key="`layout-row-${rowIndex}`"
          class="flex gap-2"
          :class="KEYBOARD_ROW_OFFSETS[rowIndex]"
        >
          <div
            v-for="(keyLabel, keyIndex) in row"
            :key="`${rowIndex}-${keyIndex}-${keyLabel}`"
            class="flex h-12 w-12 shrink-0 items-center justify-center rounded-lg border border-slate-300 bg-white font-mono text-base font-semibold text-slate-900 shadow-sm"
          >
            {{ keyLabel }}
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
