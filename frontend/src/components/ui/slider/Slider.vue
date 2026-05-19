<script setup lang="ts">
import { SliderRange, SliderRoot, SliderThumb, SliderTrack, useForwardPropsEmits } from "radix-vue";
import { cn } from "../../../lib/utils";

const props = withDefaults(
  defineProps<{
    modelValue?: number[];
    defaultValue?: number[];
    min?: number;
    max?: number;
    step?: number;
    disabled?: boolean;
    class?: string;
  }>(),
  {
    modelValue: undefined,
    defaultValue: undefined,
    min: 0,
    max: 100,
    step: 1,
    disabled: false,
    class: "",
  },
);

const emits = defineEmits<{
  "update:modelValue": [value: number[] | undefined];
}>();

const forwarded = useForwardPropsEmits(props, emits);
</script>

<template>
  <SliderRoot
    v-bind="forwarded"
    :class="cn('relative flex w-full touch-none select-none items-center', $props.class)"
  >
    <SliderTrack class="relative h-2 w-full grow overflow-hidden rounded-full bg-slate-200">
      <SliderRange class="absolute h-full bg-teal-700" />
    </SliderTrack>
    <SliderThumb
      class="block h-5 w-5 rounded-full border border-teal-700 bg-white shadow transition focus-visible:outline-none focus-visible:ring-4 focus-visible:ring-teal-100 disabled:pointer-events-none disabled:opacity-50"
    />
  </SliderRoot>
</template>
