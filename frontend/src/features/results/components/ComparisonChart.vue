<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
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

import { CostHoverMark } from '.';
import { toComparisonRows } from '../comparison';

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale);

const GREEN_COLOR = 'hsl(140, 90%, 45%)';
const RED_COLOR = 'hsl(0, 90%, 45%)';
const DEFAULT_COLOR = themeColor('--muted-foreground');

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
  return userIsBetter ? GREEN_COLOR : RED_COLOR;
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

const userChartStyle = computed(() => {
  const userTotal = props.userResult.totalCost;
  const qwertyTotal = props.qwertyResult.totalCost;
  if (userTotal === qwertyTotal) {
    return { color: DEFAULT_COLOR };
  }
  return { color: userTotal < qwertyTotal ? GREEN_COLOR : RED_COLOR };
});

const qwertyData = computed(() => ({
  labels: labels.value,
  datasets: [
    {
      label: 'QWERTY',
      data: rows.value.map((row) => row.qwertyValue),
      backgroundColor: DEFAULT_COLOR,
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
        barColor(row.userValue, row.qwertyValue, row.lowerIsBetter, DEFAULT_COLOR),
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
        <CostHoverMark :value="qwertyResult.totalCost">
          <div class="text-center">
            <p class="text-sm font-medium">Total cost</p>
            <p class="mt-1 text-2xl font-bold">
              {{ qwertyResult.totalCost.toFixed(4) }}
            </p>
          </div>
        </CostHoverMark>
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
        <CostHoverMark :value="userResult.totalCost">
          <div class="text-center">
            <p class="text-sm font-medium" :style="userChartStyle">Total cost</p>
            <p class="mt-1 text-2xl font-bold" :style="userChartStyle">
              {{ userResult.totalCost.toFixed(4) }}
            </p>
          </div>
        </CostHoverMark>
      </CardHeader>
      <CardContent class="pt-4">
        <div class="relative h-80">
          <Bar :data="userData" :options="chartOptions" />
        </div>
      </CardContent>
    </Card>
  </div>
</template>
