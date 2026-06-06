<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
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

import { useResultsStore } from '../store';

ChartJS.register(Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale);

const store = useResultsStore();
const { costHistory } = storeToRefs(store);

const charData = computed(() => ({
  labels: costHistory.value.map((_, i) => i + 1),
  datasets: [
    {
      label: 'Recent cost history',
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
    <CardHeader>
      <CardTitle>Recent cost history</CardTitle>
    </CardHeader>
    <CardContent class="pt-4">
      <div class="relative h-80">
        <Line :data="charData" :options="chartOptions" />
      </div>
    </CardContent>
  </Card>
</template>
