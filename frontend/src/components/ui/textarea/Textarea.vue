<script setup lang="ts">
import { cn } from "../../../lib/utils";

const model = defineModel<string>();

defineProps<{
  class?: string;
}>();
</script>

<template>
  <textarea
    v-model="model"
    :class="
      cn(
        'flex min-h-24 w-full rounded-md border border-slate-300 bg-white px-3 py-3 text-sm leading-6 text-slate-900 shadow-sm transition placeholder:text-slate-400 focus-visible:border-teal-500 focus-visible:outline-none focus-visible:ring-4 focus-visible:ring-teal-100 disabled:cursor-not-allowed disabled:opacity-50',
        $props.class,
      )
    "
  ></textarea>
</template>