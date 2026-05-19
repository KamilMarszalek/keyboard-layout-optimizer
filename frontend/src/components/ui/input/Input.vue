<script setup lang="ts">
import { cn } from "../../../lib/utils";

const [model, modifiers] = defineModel<string | number>({
  set(value) {
    if (modifiers.number && value !== "") {
      return Number(value);
    }
    return value;
  },
});

defineProps<{
  class?: string;
}>();
</script>

<template>
  <input
    v-model="model"
    :class="
      cn(
        'flex h-10 w-full rounded-md border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 shadow-sm transition placeholder:text-slate-400 focus-visible:border-teal-500 focus-visible:outline-none focus-visible:ring-4 focus-visible:ring-teal-100 disabled:cursor-not-allowed disabled:opacity-50',
        $props.class,
      )
    "
  />
</template>