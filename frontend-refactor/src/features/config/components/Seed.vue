<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import ConfigNumberField from './ConfigNumberField.vue';
import { seedControl } from '../config.controls';

const control = seedControl;
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Seed</CardTitle>
    </CardHeader>
    <CardContent class="pt-5">
      <ConfigNumberField
        :key="control.key"
        :name="`config.${control.key}`"
        :label="control.label"
        :description="control.description"
        :step="control.step"
      />
    </CardContent>
  </Card>
</template>
