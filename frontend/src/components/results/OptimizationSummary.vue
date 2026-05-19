<script setup lang="ts">
import { storeToRefs } from "pinia";
import { Card, CardContent, CardHeader, CardTitle } from "../ui/card";
import { formatNumber } from "../../lib/format";
import { useOptimizerStore } from "../../stores/optimizerStore";

const store = useOptimizerStore();
const { bestCost, costHistoryLength } = storeToRefs(store);
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Optimization result</CardTitle>
    </CardHeader>
    <CardContent class="grid gap-4 pt-5 sm:grid-cols-2">
      <div class="stat-row p-4">
        <p class="text-sm text-slate-600">
          Best cost
        </p>
        <p class="mt-2 text-2xl font-semibold text-slate-950">
          {{ bestCost === null ? "" : formatNumber(bestCost) }}
        </p>
      </div>
      <div class="stat-row p-4">
        <p class="text-sm text-slate-600">
          Cost history length
        </p>
        <p class="mt-2 text-2xl font-semibold text-slate-950">
          {{ costHistoryLength }}
        </p>
      </div>
    </CardContent>
  </Card>
</template>
