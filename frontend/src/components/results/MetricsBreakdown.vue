<script setup lang="ts">
import { storeToRefs } from "pinia";
import { Card, CardContent, CardHeader, CardTitle } from "../ui/card";
import { formatNumber } from "../../lib/format";
import { useOptimizerStore } from "../../stores/optimizerStore";

const store = useOptimizerStore();
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
          class="stat-row flex items-center justify-between gap-4 py-3"
        >
          <dt class="text-sm text-slate-600">
            {{ metric.label }}
          </dt>
          <dd class="text-sm font-semibold text-slate-950">
            {{ formatNumber(metric.value) }}
          </dd>
        </div>
      </dl>
    </CardContent>
  </Card>
</template>
