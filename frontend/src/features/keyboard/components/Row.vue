<script setup lang="ts">
import { type KeyView } from '../heatmap';
import Key from './Key.vue';

const props = defineProps<{
  views: KeyView[];
  offsetClass?: string;
  editable?: boolean;
  startIndex?: number;
}>();

const emit = defineEmits<{
  dragstart: [index: number];
  dragover: [event: DragEvent];
  drop: [index: number];
}>();
</script>

<template>
  <div class="flex gap-2" :class="offsetClass">
    <Key
      v-for="(view, keyIndex) in views"
      :key="`${keyIndex}-${view.mapping.base}`"
      :view="view"
      :draggable="editable"
      :key-index="(props.startIndex ?? 0) + keyIndex"
      @dragstart="emit('dragstart', $event)"
      @dragover="emit('dragover', $event)"
      @drop="emit('drop', $event)"
    />
  </div>
</template>
