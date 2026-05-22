<script setup lang="ts">
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { annealingControls } from '../config.controls';
import ConfigNumberField from './ConfigNumberField.vue';
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

    <CardContent class="grid gap-4 pt-5">
      <ConfigNumberField
        v-for="control in annealingControls"
        :key="control.key"
        :name="`config.annealing.${control.key}`"
        :label="control.label"
        :step="control.step"
        :min="control.min"
        :max="control.max"
      />
    </CardContent>
  </Card>
</template>
