import type { MetricsWeights } from '@/features/config/schema';
import { useWeightsStore } from '@/features/config/store';
import { useCorpusStore } from '@/features/corpus/store';
import type { Layout } from '@/features/keyboard/types';
import { formatError } from '@/lib/error';
import { evaluateLayout } from '@/wasm/queries';
import { defineStore } from 'pinia';

import { fromEvaluateResultDto, toEvaluateRequestDto } from './mapper';
import type { EvaluateResult } from './types';

interface EvaluatorState {
  isEvaluating: boolean;
  error: string | null;
  result: EvaluateResult | null;
  qwertyResult: EvaluateResult | null;
  evaluatedLayout: Layout | null;
  lastWeights: MetricsWeights | null;
  lastText: string | null;
}

export const useEvaluatorStore = defineStore('evaluator', {
  state: (): EvaluatorState => ({
    isEvaluating: false,
    error: null,
    result: null,
    qwertyResult: null,
    evaluatedLayout: null,
    lastWeights: null,
    lastText: null,
  }),
  getters: {
    needsReEvaluation:
      (state) =>
      (currentLayout: Layout): boolean => {
        if (!state.evaluatedLayout) {
          return false;
        }
        const evaluated = state.evaluatedLayout.mappings;
        const current = currentLayout.mappings;
        if (evaluated.length !== current.length) {
          return true;
        }
        if (current.some((mapping, index) => mapping.base !== evaluated[index].base)) {
          return true;
        }
        if (useCorpusStore().text !== state.lastText) {
          return true;
        }
        return JSON.stringify(useWeightsStore().weights) !== JSON.stringify(state.lastWeights);
      },
  },
  actions: {
    async evaluate(
      keys: string[],
      layoutSnapshot: Layout,
      text: string,
      weights: MetricsWeights,
      qwertyKeys: string[],
    ) {
      if (this.isEvaluating) {
        return;
      }

      this.isEvaluating = true;
      this.error = null;

      try {
        const [userResult, qwertyResult] = await Promise.all([
          evaluateLayout(toEvaluateRequestDto(keys, text, weights)),
          evaluateLayout(toEvaluateRequestDto(qwertyKeys, text, weights)),
        ]);
        this.result = fromEvaluateResultDto(userResult);
        this.qwertyResult = fromEvaluateResultDto(qwertyResult);

        this.evaluatedLayout = {
          mappings: layoutSnapshot.mappings.map((mapping) => ({ ...mapping })),
        };
        this.lastWeights = weights;
        this.lastText = text;
      } catch (caught) {
        this.error = formatError(caught);
      } finally {
        this.isEvaluating = false;
      }
    },
  },
});
