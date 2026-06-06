import { defineStore } from 'pinia';
import type { OptimizeRequest } from './optimizer.schema';
import { formatError } from '@/lib/format';
import {
  disposeOptimizerWorker,
  optimizeInWorker,
} from '@/services/optimizer/optimizerWorkerClient';
import { useResultsStore } from '@/features/results/results.store';
import { fromOptimizeResultDto, toOptimizeRequestDto } from './optimizer.mapper';

export const useOptimizerStore = defineStore('optimizer', {
  state: () => ({
    isOptimizing: false,
    error: null as string | null,
  }),
  actions: {
    async run(request: OptimizeRequest) {
      if (this.isOptimizing) {
        return;
      }

      this.isOptimizing = true;
      this.error = null;

      try {
        const resultsStore = useResultsStore();
        const requestDto = toOptimizeRequestDto(request);
        const result = await optimizeInWorker(requestDto);
        resultsStore.setResult(fromOptimizeResultDto(result));
      } catch (caught) {
        this.error = formatError(caught);
      } finally {
        this.isOptimizing = false;
      }
    },
    dispose() {
      disposeOptimizerWorker();
    },
  },
});
