<script setup lang="ts">
import { nextTick, ref, watch } from 'vue';

const props = defineProps<{ trigger: unknown }>();

const element = ref<HTMLElement | null>(null);

watch(
  () => props.trigger,
  async (value) => {
    if (value === null) return;
    await nextTick();
    element.value?.scrollIntoView({ behavior: 'smooth', block: 'center' });
  },
);
</script>

<template>
  <div ref="element"><slot /></div>
</template>
