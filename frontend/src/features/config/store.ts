import { defineStore } from 'pinia';

import { type MetricsWeights, defaultWeights } from './schema';

interface WeightsState {
  weights: MetricsWeights;
}

export const useWeightsStore = defineStore('weights', {
  state: (): WeightsState => ({
    weights: { ...defaultWeights },
  }),
  actions: {
    setWeights(weights: MetricsWeights) {
      this.weights = { ...weights };
    },
  },
});
