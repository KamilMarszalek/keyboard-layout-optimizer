<script setup lang="ts">
import Header from '@/components/common/Header.vue';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { useWeightsStore } from '@/features/config/store';
import { useCorpusStore } from '@/features/corpus/store';
import { useEvaluatorStore } from '@/features/evaluator/store';
import { KeyboardPreview } from '@/features/keyboard/components';
import { useKeyboardStore } from '@/features/keyboard/store';
import { useModeStore } from '@/features/mode/store';
import { OptimizerForm } from '@/features/optimizer/components';
import { CostHistory, MetricsBreakdown, OptimizationResult } from '@/features/results/components';
import { useResultsStore } from '@/features/results/store';
import { storeToRefs } from 'pinia';

const resultStore = useResultsStore();
const { result } = storeToRefs(resultStore);

const evaluatorStore = useEvaluatorStore();
const { result: evaluateResult } = storeToRefs(evaluatorStore);

const corpusStore = useCorpusStore();
const keyboardStore = useKeyboardStore();
const weightsStore = useWeightsStore();

const modeStore = useModeStore();
const { mode } = storeToRefs(modeStore);

function handleReEvaluate() {
  const keys = keyboardStore.editableLayout.mappings.map((mapping) => mapping.base);
  void evaluatorStore.evaluate(keys, corpusStore.text, weightsStore.weights);
}
</script>

<template>
  <main class="min-h-screen px-4 py-8 sm:px-6 lg:px-8">
    <div class="mx-auto max-w-7xl space-y-6">
      <Header />
      <OptimizerForm />
      <KeyboardPreview
        :editable="mode === 'evaluate'"
        :on-re-evaluate="mode === 'evaluate' ? handleReEvaluate : undefined"
      />

      <section
        v-if="mode === 'optimize' && result"
        class="grid gap-6 lg:grid-cols-[minmax(0,1fr)_minmax(360px,0.8fr)]"
      >
        <OptimizationResult />
        <div class="space-y-6">
          <MetricsBreakdown :metrics="result.metrics" />
          <CostHistory />
        </div>
      </section>

      <section
        v-if="mode === 'evaluate' && evaluateResult"
        class="grid gap-6 lg:grid-cols-[minmax(0,1fr)_minmax(360px,0.8fr)]"
      >
        <Card>
          <CardHeader>
            <CardTitle>Evaluation result</CardTitle>
          </CardHeader>
          <CardContent class="pt-5">
            <p class="text-sm">Total cost</p>
            <p class="mt-2 text-2xl">{{ evaluateResult.totalCost.toFixed(4) }}</p>
          </CardContent>
        </Card>
        <div class="space-y-6">
          <MetricsBreakdown :metrics="evaluateResult.metrics" />
        </div>
      </section>
    </div>
  </main>
</template>
