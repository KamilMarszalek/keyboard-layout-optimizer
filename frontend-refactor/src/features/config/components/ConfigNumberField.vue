<script setup lang="ts">
import { useField } from 'vee-validate';
import { Input } from '@/components/ui/input';
import ConfigField from './ConfigField.vue';

const props = defineProps<{
  name: string;
  label: string;
  description?: string;
  step?: number;
  min?: number;
  max?: number;
}>();

const { value, errorMessage } = useField<number>(() => props.name);
</script>

<template>
  <ConfigField :id="name" :label="label" :description="description" :error="errorMessage">
    <Input
      :id="name"
      :model-value="value"
      type="number"
      :step="step"
      :min="min"
      :max="max"
      @update:model-value="(v) => (value = Number(v))"
    />
  </ConfigField>
</template>
