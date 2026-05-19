<script setup lang="ts">
import { cn } from "../../../lib/utils";

defineProps<{
  class?: string;
}>();
</script>

<template>
  <div :class="cn('p-5 sm:p-6', $props.class)">
    <slot></slot>
  </div>
</template>
