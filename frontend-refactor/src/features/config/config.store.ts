import { defineStore } from 'pinia';
import { defaultMetricWeithgs, defaultAnnealingParams } from './config.schema';

export const useConfigStore = defineStore('config', {
  state: () => {
    return {
      metricWeights: defaultMetricWeithgs,
      annealingParams: defaultAnnealingParams,
      seed: 42,
    };
  },
});
