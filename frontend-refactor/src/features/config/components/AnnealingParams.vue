<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { annealingControls } from '../config.controls';
import { useConfigStore } from '../config.store';
import ConfigNumberField from './ConfigNumberField.vue';

const store = useConfigStore();
const { annealingParams } = storeToRefs(store);
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Annealing parameters</CardTitle>
      <CardDescription class="mt-1">
        Control the cooling schedule and work performed at each temperature. Higher iteration counts
        can take longer in the browser.
      </CardDescription>
    </CardHeader>

    <CardContent class="grid gap-4 pt-5 sm:grid-cols-2 lg:grid-cols-4">
      <ConfigNumberField
        v-for="control in annealingControls"
        v-model="annealingParams[control.key]"
        :key="control.key"
        :id="`annealing-${control.key}`"
        :label="control.label"
        :step="control.step"
        :min="control.min"
        :max="control.max"
      />
    </CardContent>
  </Card>
</template>
