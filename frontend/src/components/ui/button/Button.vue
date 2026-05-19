<script setup lang="ts">
import { Primitive, type PrimitiveProps } from "radix-vue";
import { computed } from "vue";
import { cn } from "../../../lib/utils";

const props = withDefaults(
  defineProps<
    PrimitiveProps & {
      variant?: "default" | "outline" | "ghost";
      size?: "default" | "sm" | "lg" | "icon";
      class?: string;
    }
  >(),
  {
    as: "button",
    variant: "default",
    size: "default",
    class: "",
  },
);

const classes = computed(() =>
  cn(
    "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium transition focus-visible:outline-none focus-visible:ring-4 disabled:pointer-events-none disabled:opacity-50",
    props.variant === "default" &&
      "bg-teal-700 text-white shadow-sm hover:bg-teal-800 focus-visible:ring-teal-200",
    props.variant === "outline" &&
      "border border-slate-200 bg-white text-slate-900 shadow-sm hover:bg-slate-50 focus-visible:ring-slate-200",
    props.variant === "ghost" &&
      "text-slate-700 hover:bg-slate-100 hover:text-slate-950 focus-visible:ring-slate-200",
    props.size === "default" && "h-10 px-4 py-2",
    props.size === "sm" && "h-9 px-3",
    props.size === "lg" && "h-12 px-5",
    props.size === "icon" && "h-10 w-10",
    props.class,
  ),
);
</script>

<template>
  <Primitive
    :as="as"
    :as-child="asChild"
    :class="classes"
  >
    <slot></slot>
  </Primitive>
</template>
