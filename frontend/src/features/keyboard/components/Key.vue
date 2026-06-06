<script setup lang="ts">
import type { KeyView } from '../heatmap';

const props = defineProps<{
  view: KeyView;
  draggable?: boolean;
  keyIndex?: number;
}>();

const emit = defineEmits<{
  dragstart: [index: number];
  dragover: [event: DragEvent];
  drop: [index: number];
}>();

function onDragStart() {
  if (props.keyIndex !== undefined) {
    emit('dragstart', props.keyIndex);
  }
}

function onDragOver(event: DragEvent) {
  emit('dragover', event);
}

function onDrop() {
  if (props.keyIndex !== undefined) {
    emit('drop', props.keyIndex);
  }
}
</script>

<template>
  <div
    class="relative flex h-12 w-12 shrink-0 items-center justify-center rounded-md border border-border bg-card font-mono text-base font-semibold text-card-foreground shadow-sm"
    :class="draggable ? 'cursor-grab active:cursor-grabbing' : ''"
    :style="view.style"
    :draggable="draggable"
    @dragstart="onDragStart"
    @dragover="onDragOver"
    @drop="onDrop"
  >
    <span
      v-if="view.mapping.shifted"
      class="absolute right-1 top-1 text-[0.625rem] font-medium leading-none text-muted-foreground"
    >
      {{ view.mapping.shifted }}
    </span>
    {{ view.mapping.base }}
  </div>
</template>
