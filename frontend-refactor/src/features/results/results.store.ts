import { defineStore } from 'pinia';
import type { OptimizeResult } from './results.types';
import { metricControls } from '@/features/config/config.controls';

interface ResultsState {
  result?: OptimizeResult;
  error?: string;
}

export const useResultsStore = defineStore('results', {
  state: (): ResultsState => ({
    result: undefined,
    error: undefined,
  }),
  actions: {
    setError(e: unknown) {
      this.error = e instanceof Error ? e.message : String(e);
    },
  },
  getters: {
    costHistory: (state) => state.result!.costHistory,
    recentCostHistory: (state) => state.result!.costHistory.slice(-8),
    resultMetrics: (state) =>
      metricControls.map((m) => ({
        key: m.key,
        label: m.label,
        value: state.result!.metrics[m.key],
      })),
  },
});
