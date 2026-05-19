<script setup lang="ts">
import { Label } from "radix-vue";
import { cn } from "../../../lib/utils";

defineProps<{
  class?: string;
}>();
</script>

<template>
  <Label :class="cn('text-sm font-medium leading-none text-slate-700', $props.class)">
    <slot></slot>
  </Label>
</template>
