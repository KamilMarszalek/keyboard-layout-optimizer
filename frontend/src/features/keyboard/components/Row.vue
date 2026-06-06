<script setup lang="ts">
import { type KeyView } from '../heatmap';
import Key from './Key.vue';

defineProps<{
  views: KeyView[];
  offsetClass?: string;
}>();
</script>

<template>
  <div class="flex gap-2" :class="offsetClass">
    <Key v-for="(view, keyIndex) in views" :key="`${keyIndex}-${view.mapping.base}`" :view="view" />
  </div>
</template>
