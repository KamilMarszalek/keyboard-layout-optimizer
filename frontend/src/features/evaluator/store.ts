import type { MetricsWeights } from '@/features/config/schema';
import { useKeyboardStore } from '@/features/keyboard/store';
import type { Layout } from '@/features/keyboard/types';
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
    evaluatedLayout: null as Layout | null,
    lastWeights: null as MetricsWeights | null,
  }),
  getters: {
    layoutChanged(state): boolean {
      if (!state.evaluatedLayout) {
        return false;
      }
      const keyboard = useKeyboardStore();
      const evaluated = state.evaluatedLayout.mappings;
      const current = keyboard.editableLayout.mappings;
      if (evaluated.length !== current.length) {
        return true;
      }
      return current.some((mapping, index) => mapping.base !== evaluated[index].base);
    },
  },
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

        const keyboard = useKeyboardStore();
        this.evaluatedLayout = {
          mappings: keyboard.editableLayout.mappings.map((mapping) => ({ ...mapping })),
        };
        this.lastWeights = weights;
      } catch (caught) {
        this.error = formatError(caught);
      } finally {
        this.isEvaluating = false;
      }
    },
  },
});
