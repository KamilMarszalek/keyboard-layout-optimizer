<script setup lang="ts">
import { cn } from "../../../lib/utils";

withDefaults(
  defineProps<{
    variant?: "default" | "secondary";
    class?: string;
  }>(),
  {
    variant: "default",
    class: "",
  },
);
</script>

<template>
  <span
    :class="
      cn(
        'inline-flex items-center rounded-full px-3 py-1 text-sm font-medium',
        variant === 'default' && 'bg-teal-50 text-teal-800 ring-1 ring-teal-200',
        variant === 'secondary' && 'bg-slate-100 text-slate-600',
        $props.class,
      )
    "
  >
    <slot></slot>
  </span>
</template>
