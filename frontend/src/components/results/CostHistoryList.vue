<script setup lang="ts">
import { storeToRefs } from "pinia";
import { Card, CardContent, CardHeader, CardTitle } from "../ui/card";
import { formatNumber } from "../../lib/format";
import { useOptimizerStore } from "../../stores/optimizerStore";

const store = useOptimizerStore();
const { costHistoryLength, recentCostHistory } = storeToRefs(store);
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
          class="stat-row flex items-center justify-between gap-4 py-2 text-sm"
        >
          <span class="text-slate-500">
            #{{ costHistoryLength - recentCostHistory.length + index + 1 }}
          </span>
          <span class="font-medium text-slate-950">{{ formatNumber(value) }}</span>
        </li>
      </ol>
    </CardContent>
  </Card>
</template>
