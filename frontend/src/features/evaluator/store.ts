import type { MetricsWeights } from '@/features/config/schema';
import { formatError } from '@/lib/error';
import type { EvaluateResultDto } from '@/wasm/dto';
import { evaluateLayout } from '@/wasm/queries';
import { defineStore } from 'pinia';

import { toEvaluateRequestDto } from './mapper';

export const useEvaluatorStore = defineStore('evaluator', {
  state: () => ({
    isEvaluating: false,
    error: null as string | null,
    result: null as EvaluateResultDto | null,
  }),
  actions: {
    async evaluate(keys: string[], text: string, weights: MetricsWeights) {
      if (this.isEvaluating) {
        return;
      }

      this.isEvaluating = true;
      this.error = null;

      try {
        const requestDto = toEvaluateRequestDto(keys, text, weights);
        this.result = await evaluateLayout(requestDto);
      } catch (caught) {
        this.error = formatError(caught);
      } finally {
        this.isEvaluating = false;
      }
    },
  },
});
