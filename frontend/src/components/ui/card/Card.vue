<script setup lang="ts">
import { cn } from "../../../lib/utils";

defineProps<{
  class?: string;
}>();
</script>

<template>
  <div :class="cn('rounded-lg bg-white text-slate-950 shadow-sm ring-1 ring-slate-200', $props.class)">
    <slot></slot>
  </div>
</template>
