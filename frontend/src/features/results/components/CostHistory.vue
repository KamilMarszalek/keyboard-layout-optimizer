<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { storeToRefs } from 'pinia';

import { useResultsStore } from '../results.store';

const store = useResultsStore();
const { costHistory, recentCostHistory } = storeToRefs(store);
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Recent cost history</CardTitle>
    </CardHeader>
    <CardContent class="pt-4">
      <ol class="space-y-2">
        <li
          v-for="(value, index) in recentCostHistory"
          :key="index"
          class="flex items-center justify-between gap-4 py-2 text-sm"
        >
          <span> #{{ costHistory.length - recentCostHistory.length + index + 1 }} </span>
          <span>{{ value.toFixed(4) }}</span>
        </li>
      </ol>
    </CardContent>
  </Card>
</template>
