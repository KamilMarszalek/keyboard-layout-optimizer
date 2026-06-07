<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { HoverCard, HoverCardContent, HoverCardTrigger } from '@/components/ui/hover-card';
import { themeColor } from '@/lib/token';
import {
  CategoryScale,
  Chart as ChartJS,
  Legend,
  LineElement,
  LinearScale,
  PointElement,
  Title,
  Tooltip,
} from 'chart.js';
import { storeToRefs } from 'pinia';
import { computed } from 'vue';
import { Line } from 'vue-chartjs';

import { useOptimizerResultStore } from '../store';
import OptimizationResult from './OptimizationResult.vue';

ChartJS.register(Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale);

const store = useOptimizerResultStore();
const { costHistory } = storeToRefs(store);

const chartData = computed(() => ({
  labels: costHistory.value.map((_, i) => i + 1),
  datasets: [
    {
      label: 'Cost history',
      data: costHistory.value.map((x) => Number(x.toFixed(4))),
      fill: false,
      borderColor: themeColor('--chart-1'),
      borderWidth: 2,
      pointRadius: 0,
      pointHoverRadius: 0,
      tension: 0.1,
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
      title: {
        display: true,
        text: 'Iterations',
      },
    },
    y: {
      title: {
        display: true,
        text: 'Cost',
      },
    },
  },
};
</script>

<template>
  <Card>
    <CardHeader class="flex flex-row items-center justify-between space-y-0">
      <CardTitle>Cost history</CardTitle>
      <HoverCard :open-delay="100" :close-delay="100">
        <HoverCardTrigger
          as="button"
          type="button"
          aria-label="Optimization result"
          class="flex h-6 w-6 items-center justify-center rounded-full border border-border text-xs font-bold text-muted-foreground hover:bg-muted"
        >
          ?
        </HoverCardTrigger>
        <HoverCardContent align="end" class="w-72">
          <OptimizationResult />
        </HoverCardContent>
      </HoverCard>
    </CardHeader>
    <CardContent class="pt-4">
      <div class="relative h-80">
        <Line :data="chartData" :options="chartOptions" />
      </div>
    </CardContent>
  </Card>
</template>
