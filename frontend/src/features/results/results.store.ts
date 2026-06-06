import { defineStore } from 'pinia';

import type { OptimizeResult } from './results.types';

const RECENT_COST_HISTORY_SIZE = 8;

interface ResultsState {
  result?: OptimizeResult;
}

export const useResultsStore = defineStore('results', {
  state: (): ResultsState => ({
    result: undefined,
  }),
  actions: {
    setResult(result: OptimizeResult) {
      this.result = result;
    },
  },
  getters: {
    costHistory: (state) => state.result?.costHistory ?? [],
    recentCostHistory: (state) => state.result?.costHistory.slice(-RECENT_COST_HISTORY_SIZE) ?? [],
    resultMetrics: (state) => state.result?.metrics,
  },
});
