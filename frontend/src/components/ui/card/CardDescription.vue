<script setup lang="ts">
import { cn } from "../../../lib/utils";

defineProps<{
  class?: string;
}>();
</script>

<template>
  <p :class="cn('text-sm leading-6 text-slate-600', $props.class)">
    <slot></slot>
  </p>
</template>
