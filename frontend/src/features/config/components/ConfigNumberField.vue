<script setup lang="ts">
import {
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
} from '@/components/ui/number-field';
import { useField } from 'vee-validate';

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
    <NumberField v-model="value" :id="name" :step="step" :min="min" :max="max">
      <NumberFieldContent>
        <NumberFieldDecrement />
        <NumberFieldInput />
        <NumberFieldIncrement />
      </NumberFieldContent>
    </NumberField>
  </ConfigField>
</template>
