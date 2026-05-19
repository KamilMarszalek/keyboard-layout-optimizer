<script setup lang="ts">
import { cn } from "../../../lib/utils";

withDefaults(
  defineProps<{
    variant?: "default" | "destructive" | "warning";
    class?: string;
  }>(),
  {
    variant: "default",
    class: "",
  },
);
</script>

<template>
  <div
    role="alert"
    :class="
      cn(
        'rounded-lg border px-4 py-3 text-sm',
        variant === 'default' && 'border-slate-200 bg-slate-50 text-slate-700',
        variant === 'destructive' && 'border-red-200 bg-red-50 text-red-700',
        variant === 'warning' && 'border-amber-200 bg-amber-50 text-amber-800',
        $props.class,
      )
    "
  >
    <slot></slot>
  </div>
</template>
