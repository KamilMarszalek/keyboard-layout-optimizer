<script setup lang="ts">
import { cn } from "../../../lib/utils";

defineProps<{
  class?: string;
}>();
</script>

<template>
  <div :class="cn('flex flex-col space-y-1.5 p-5 pb-0 sm:p-6 sm:pb-0', $props.class)">
    <slot></slot>
  </div>
</template>
