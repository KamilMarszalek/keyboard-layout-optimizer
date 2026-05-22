<script setup lang="ts">
import { Input } from '@/components/ui/input';
import ConfigField from './ConfigField.vue';

defineProps<{
  id: string;
  label: string;
  description?: string;
  step?: number;
  min?: number;
  max?: number;
}>();

const model = defineModel<number>({ required: true });
</script>

<template>
  <ConfigField :id="id" :label="label" :description="description">
    <Input :id="id" v-model.number="model" type="number" :step="step" :min="min" :max="max" />
  </ConfigField>
</template>
