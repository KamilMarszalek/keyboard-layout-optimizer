<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

import { seedControl } from '../config.controls.ts';
import ConfigNumberField from './ConfigNumberField.vue';
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Seed</CardTitle>
    </CardHeader>
    <CardContent class="pt-5">
      <ConfigNumberField
        :key="seedControl.key"
        :name="`${seedControl.key}`"
        :label="seedControl.label"
        :description="seedControl.description"
        :step="seedControl.step"
      />
    </CardContent>
  </Card>
</template>
