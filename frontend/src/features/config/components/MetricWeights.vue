<script setup lang="ts">
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { metricControls } from '../config.controls.ts';
import ConfigSliderField from './ConfigSliderField.vue';
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Metric weights</CardTitle>
      <CardDescription class="mt-1"> Set each objective from 0.0 to 5.0. </CardDescription>
    </CardHeader>

    <CardContent class="grid gap-4 pt-5">
      <ConfigSliderField
        v-for="control in metricControls"
        :key="control.key"
        :name="`config.weights.${control.key}`"
        :label="control.label"
        :description="control.description"
        :step="control.step"
        :min="control.min"
        :max="control.max"
      />
    </CardContent>
  </Card>
</template>
