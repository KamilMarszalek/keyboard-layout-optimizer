<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import ConfigNumberField from './ConfigNumberField.vue';
import { seedControl } from '../config.controls';
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Seed</CardTitle>
    </CardHeader>
    <CardContent class="pt-5">
      <ConfigNumberField
        :key="seedControl.key"
        :name="`config.${seedControl.key}`"
        :label="seedControl.label"
        :description="seedControl.description"
        :step="seedControl.step"
      />
    </CardContent>
  </Card>
</template>
