<script setup lang="ts">
import { Badge } from '@/components/ui/badge';
import { Slider } from '@/components/ui/slider';
import { useField } from 'vee-validate';

import ConfigField from './ConfigField.vue';

const props = defineProps<{
  name: string;
  label: string;
  description?: string;
  min?: number;
  max?: number;
  step?: number;
}>();

const { value, errorMessage } = useField<number>(() => props.name);
</script>

<template>
  <ConfigField :id="name" :label="label" :description="description" :error="errorMessage">
    <template #value>
      <Badge variant="secondary">{{ value.toFixed(1) }}</Badge>
    </template>
    <Slider
      :model-value="[value]"
      :id="name"
      :min="min"
      :max="max"
      :step="step"
      @update:model-value="(v) => (value = v?.[0] ?? 0)"
    />
  </ConfigField>
</template>
