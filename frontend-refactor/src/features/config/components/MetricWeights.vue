<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { metricControls } from '../config.controls';
import { useConfigStore } from '../config.store';
import ConfigSliderField from './ConfigSliderField.vue';

const store = useConfigStore();
const { metricWeights } = storeToRefs(store);
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Metric weights</CardTitle>
      <CardDescription class="mt-1"> Set each objective from 0.0 to 5.0. </CardDescription>
    </CardHeader>

    <CardContent class="grid gap-4 pt-5 md:grid-cols-2">
      <ConfigSliderField
        v-for="metric in metricControls"
        v-model="metricWeights[metric.key]"
        :key="metric.key"
        :id="`weight-${metric.key}`"
        :label="metric.label"
        :description="metric.description"
        :step="metric.step"
        :min="metric.min"
        :max="metric.max"
      />
    </CardContent>
  </Card>
</template>
