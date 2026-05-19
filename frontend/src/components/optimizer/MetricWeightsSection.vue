<script setup lang="ts">
import { storeToRefs } from "pinia";
import { computed } from "vue";
import { Badge } from "../ui/badge";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "../ui/card";
import { Label } from "../ui/label";
import { Slider } from "../ui/slider";
import { metricControls } from "../../constants/optimizerControls";
import { useOptimizerStore } from "../../stores/optimizerStore";
import type { MetricWeightsDto } from "../../wasmTypes";

const store = useOptimizerStore();
const { metricWeights } = storeToRefs(store);

function updateWeight(key: keyof MetricWeightsDto, value: number[] | undefined) {
  metricWeights.value[key] = value?.[0] ?? 0;
}

const controls = computed(() =>
  metricControls.map((metric) => ({
    ...metric,
    value: metricWeights.value[metric.key],
  })),
);
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Metric weights</CardTitle>
      <CardDescription class="mt-1">
        Set each objective from 0.0 to 5.0.
      </CardDescription>
    </CardHeader>

    <CardContent class="grid gap-4 pt-5 md:grid-cols-2">
      <div
        v-for="metric in controls"
        :key="metric.key"
        class="rounded-lg border border-slate-200 bg-slate-50 p-4"
      >
        <div class="flex items-start justify-between gap-4">
          <div>
            <Label :for="`weight-${metric.key}`" class="text-slate-800">
              {{ metric.label }}
            </Label>
            <p class="mt-1 text-xs leading-5 text-slate-500">
              {{ metric.help }}
            </p>
          </div>
          <Badge
            variant="secondary"
            class="rounded-md bg-white font-semibold text-slate-800 ring-1 ring-slate-200"
          >
            {{ metric.value.toFixed(1) }}
          </Badge>
        </div>
        <Slider
          :id="`weight-${metric.key}`"
          :model-value="[metric.value]"
          :min="0"
          :max="5"
          :step="0.1"
          class="mt-4"
          @update:model-value="(value) => updateWeight(metric.key, value)"
        />
      </div>
    </CardContent>
  </Card>
</template>
