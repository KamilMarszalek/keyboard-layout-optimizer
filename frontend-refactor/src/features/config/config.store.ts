import { defineStore } from 'pinia';
import {
  defaultMetricWeithgs,
  defaultAnnealingParams,
  type MetricsWeights,
  type AnnealingParams,
} from './config.schema';

interface ConfigState {
  metricWeights: MetricsWeights;
  annealingParams: AnnealingParams;
  seed: number | null;
}

export const useConfigStore = defineStore('config', {
  state: (): ConfigState => {
    return {
      metricWeights: defaultMetricWeithgs,
      annealingParams: defaultAnnealingParams,
      seed: null,
    };
  },
});
