<script setup lang="ts">
import { cn } from "../../../lib/utils";

defineProps<{
  class?: string;
}>();
</script>

<template>
  <h2 :class="cn('text-xl font-semibold leading-none tracking-normal text-slate-950', $props.class)">
    <slot></slot>
  </h2>
</template>
