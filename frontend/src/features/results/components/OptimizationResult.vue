<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { storeToRefs } from 'pinia';

import { useResultsStore } from '../results.store';

const store = useResultsStore();
const { result, costHistory } = storeToRefs(store);
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Optimization result</CardTitle>
    </CardHeader>
    <CardContent class="grid gap-4 pt-5 sm:grid-cols-2">
      <div class="p-4">
        <p class="text-sm">Best cost</p>
        <p class="mt-2 text-2xl">
          {{ result?.bestCost.toFixed(4) ?? '-' }}
        </p>
      </div>
      <div class="p-4">
        <p class="text-sm">Cost history length</p>
        <p class="mt-2 text-2xl">{{ costHistory.length }}</p>
      </div>
    </CardContent>
  </Card>
</template>
