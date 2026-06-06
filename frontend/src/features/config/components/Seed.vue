<script setup lang="ts">
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

import { seedControl } from '../config.controls.ts';
import ConfigNumberField from './ConfigNumberField.vue';
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Seed</CardTitle>
      <CardDescription>{{ seedControl.description }}</CardDescription>
    </CardHeader>
    <CardContent class="pt-5">
      <ConfigNumberField
        :key="seedControl.key"
        :name="`${seedControl.key}`"
        :label="seedControl.label"
        :step="seedControl.step"
      />
    </CardContent>
  </Card>
</template>
