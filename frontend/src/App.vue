<script setup lang="ts">
import Header from '@/components/common/Header.vue';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { useEvaluatorStore } from '@/features/evaluator/store';
import { useReEvaluate } from '@/features/evaluator/useReEvaluate';
import { KeyboardPreview } from '@/features/keyboard/components';
import { useModeStore } from '@/features/mode/store';
import { OptimizerForm } from '@/features/optimizer/components';
import { CostHistory, MetricsBreakdown } from '@/features/results/components';
import { useResultsStore } from '@/features/results/store';
import { storeToRefs } from 'pinia';

const resultStore = useResultsStore();
const { result } = storeToRefs(resultStore);

const evaluatorStore = useEvaluatorStore();
const { result: evaluateResult } = storeToRefs(evaluatorStore);

const modeStore = useModeStore();
const { mode } = storeToRefs(modeStore);

const { handleReEvaluate } = useReEvaluate();
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
        class="grid gap-6 lg:grid-cols-2"
      >
        
          <MetricsBreakdown :metrics="result.metrics" />
          <CostHistory />
      </section>

      <section
        v-if="mode === 'evaluate' && evaluateResult"
        class="grid gap-6 lg:grid-cols-2"
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
