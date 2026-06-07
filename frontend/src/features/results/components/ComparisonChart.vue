<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { HoverCard, HoverCardContent, HoverCardTrigger } from '@/components/ui/hover-card';
import type { EvaluateResult } from '@/features/evaluator/types';
import { themeColor } from '@/lib/token';
import {
  BarElement,
  CategoryScale,
  Chart as ChartJS,
  Legend,
  LinearScale,
  Title,
  Tooltip,
} from 'chart.js';
import { computed } from 'vue';
import { Bar } from 'vue-chartjs';

import { toComparisonRows } from '../comparison';

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale);

const GREEN = 'hsl(140, 90%, 45%)';
const RED = 'hsl(0, 90%, 45%)';

function barColor(
  userValue: number,
  qwertyValue: number,
  lowerIsBetter: boolean,
  gray: string,
): string {
  if (Math.abs(userValue - qwertyValue) < 1e-9) {
    return gray;
  }
  const userIsBetter = lowerIsBetter ? userValue < qwertyValue : userValue > qwertyValue;
  return userIsBetter ? GREEN : RED;
}

const props = defineProps<{
  userResult: EvaluateResult;
  qwertyResult: EvaluateResult;
}>();

const rows = computed(() => toComparisonRows(props.userResult, props.qwertyResult));

const labels = computed(() => rows.value.map((row) => row.label.split(' ')));

const yBounds = computed(() => {
  const values = rows.value.flatMap((row) => [row.userValue, row.qwertyValue]);
  return {
    min: Math.min(0, ...values),
    max: Math.max(...values),
  };
});

const userTotalClass = computed(() => {
  const { totalCost: userTotal } = props.userResult;
  const { totalCost: qwertyTotal } = props.qwertyResult;
  if (userTotal < qwertyTotal) {
    return 'text-green-600';
  }
  if (userTotal > qwertyTotal) {
    return 'text-destructive';
  }
  return 'text-muted-foreground';
});

const qwertyData = computed(() => ({
  labels: labels.value,
  datasets: [
    {
      label: 'QWERTY',
      data: rows.value.map((row) => row.qwertyValue),
      backgroundColor: themeColor('--muted-foreground'),
      borderRadius: 4,
    },
  ],
}));

const userData = computed(() => ({
  labels: labels.value,
  datasets: [
    {
      label: 'Your layout',
      data: rows.value.map((row) => row.userValue),
      backgroundColor: rows.value.map((row) =>
        barColor(
          row.userValue,
          row.qwertyValue,
          row.lowerIsBetter,
          themeColor('--muted-foreground'),
        ),
      ),
      borderRadius: 4,
    },
  ],
}));

const chartOptions = computed(() => ({
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      display: false,
    },
  },
  scales: {
    x: {
      ticks: {
        maxRotation: 0,
        minRotation: 0,
        autoSkip: false,
      },
    },
    y: {
      min: yBounds.value.min,
      max: yBounds.value.max,
      title: {
        display: true,
        text: 'Cost contribution',
      },
    },
  },
}));
</script>

<template>
  <div class="grid grid-cols-2 gap-6">
    <Card>
      <CardHeader class="flex flex-row items-center justify-between space-y-0">
        <CardTitle>QWERTY baseline</CardTitle>
        <HoverCard :open-delay="100" :close-delay="100">
          <HoverCardTrigger
            as="button"
            type="button"
            aria-label="QWERTY total cost"
            class="flex h-6 w-6 items-center justify-center rounded-full border border-border text-xs font-bold text-muted-foreground hover:bg-muted"
          >
            ?
          </HoverCardTrigger>
          <HoverCardContent align="end" class="w-72">
            <div class="text-center">
              <p class="text-sm font-medium">Total cost</p>
              <p class="mt-1 text-2xl font-bold">{{ qwertyResult.totalCost.toFixed(4) }}</p>
            </div>
          </HoverCardContent>
        </HoverCard>
      </CardHeader>
      <CardContent class="pt-4">
        <div class="relative h-80">
          <Bar :data="qwertyData" :options="chartOptions" />
        </div>
      </CardContent>
    </Card>
    <Card>
      <CardHeader class="flex flex-row items-center justify-between space-y-0">
        <CardTitle>Your layout</CardTitle>
        <HoverCard :open-delay="100" :close-delay="100">
          <HoverCardTrigger
            as="button"
            type="button"
            aria-label="Your layout total cost"
            class="flex h-6 w-6 items-center justify-center rounded-full border border-border text-xs font-bold text-muted-foreground hover:bg-muted"
          >
            ?
          </HoverCardTrigger>
          <HoverCardContent align="end" class="w-72">
            <div class="text-center">
              <p class="text-sm font-medium">Total cost</p>
              <p class="mt-1 text-2xl font-bold" :class="userTotalClass">
                {{ userResult.totalCost.toFixed(4) }}
              </p>
            </div>
          </HoverCardContent>
        </HoverCard>
      </CardHeader>
      <CardContent class="pt-4">
        <div class="relative h-80">
          <Bar :data="userData" :options="chartOptions" />
        </div>
      </CardContent>
    </Card>
  </div>
</template>
