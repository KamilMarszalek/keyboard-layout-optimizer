import { useWeightsStore } from '@/features/config/store';
import { useCorpusStore } from '@/features/corpus/store';
import { useKeyboardStore } from '@/features/keyboard/store';
import { computed } from 'vue';

import { useEvaluatorStore } from './store';

export function useReEvaluate() {
  const corpusStore = useCorpusStore();
  const keyboardStore = useKeyboardStore();
  const weightsStore = useWeightsStore();
  const evaluatorStore = useEvaluatorStore();

  function handleReEvaluate() {
    const layout = keyboardStore.editableLayout;
    const keys = layout.mappings.map((mapping) => mapping.base);
    void evaluatorStore.evaluate(keys, layout, corpusStore.text, weightsStore.weights);
  }

  const needsReEvaluation = computed(() =>
    evaluatorStore.needsReEvaluation(keyboardStore.editableLayout),
  );

  return { handleReEvaluate, needsReEvaluation };
}
