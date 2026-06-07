<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
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

import { metricLabels } from '../controls';
import type { MetricBreakdown } from '../types';

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale);

const props = defineProps<{
  metrics: MetricBreakdown;
}>();

const chartData = computed(() => ({
  labels: metricLabels.map((m) => m.label.split(' ')),
  datasets: [
    {
      label: 'Metric value',
      data: metricLabels.map((m) => Number(props.metrics?.[m.key] ?? 0)),
      backgroundColor: metricLabels.map((_, i) => themeColor(`--chart-${i + 1}`)),
      borderRadius: 4,
    },
  ],
}));

const chartOptions = {
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
      title: {
        display: true,
        text: 'Cost contribution',
      },
    },
  },
};
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Metrics breakdown</CardTitle>
    </CardHeader>
    <CardContent class="pt-4">
      <div class="relative h-80">
        <Bar :data="chartData" :options="chartOptions" />
      </div>
    </CardContent>
  </Card>
</template>
