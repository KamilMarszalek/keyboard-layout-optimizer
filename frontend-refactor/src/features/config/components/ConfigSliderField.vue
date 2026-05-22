<script setup lang="ts">
import { Badge } from '@/components/ui/badge';
import { Slider } from '@/components/ui/slider';
import ConfigField from './ConfigField.vue';

defineProps<{
  id: string;
  label: string;
  description?: string;
  min?: number;
  max?: number;
  step?: number;
}>();

const model = defineModel<number>({ required: true });
</script>

<template>
  <ConfigField :id="id" :label="label" :description="description">
    <template #value>
      <Badge variant="secondary">{{ model.toFixed(1) }}</Badge>
    </template>
    <Slider
      :id="id"
      :model-value="[model]"
      :min="min"
      :max="max"
      :step="step"
      @update:model-value="(value) => (model = value?.[0] ?? 0)"
    />
  </ConfigField>
</template>
