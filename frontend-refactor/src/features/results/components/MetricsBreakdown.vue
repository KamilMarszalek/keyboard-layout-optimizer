<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { useResultsStore } from '../results.store';

const store = useResultsStore();
const { resultMetrics } = storeToRefs(store);
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Metrics breakdown</CardTitle>
    </CardHeader>
    <CardContent class="pt-4">
      <dl class="space-y-3">
        <div
          v-for="metric in resultMetrics"
          :key="metric.key"
          class="flex items-center justify-between gap-4 py-3"
        >
          <dt class="text-sm">{{ metric.label }}</dt>
          <dd class="text-sm">{{ metric.value.toFixed(4) }}</dd>
        </div>
      </dl>
    </CardContent>
  </Card>
</template>
