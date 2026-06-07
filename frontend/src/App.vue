<script setup lang="ts">
import Footer from '@/components/common/Footer.vue';
import Header from '@/components/common/Header.vue';
import ScrollIntoView from '@/components/common/ScrollIntoView.vue';
import { useEvaluatorStore } from '@/features/evaluator/store';
import { useReEvaluate } from '@/features/evaluator/useReEvaluate';
import { KeyboardPreview } from '@/features/keyboard/components';
import { useModeStore } from '@/features/mode/store';
import { OptimizerForm } from '@/features/optimizer/components';
import { ComparisonChart, CostHistory, MetricsBreakdown } from '@/features/results/components';
import { useOptimizerResultStore } from '@/features/results/store';
import { storeToRefs } from 'pinia';
import { ref, watch } from 'vue';

const resultStore = useOptimizerResultStore();
const { result } = storeToRefs(resultStore);

const evaluatorStore = useEvaluatorStore();
const { result: evaluateResult, qwertyResult } = storeToRefs(evaluatorStore);

const modeStore = useModeStore();
const { mode } = storeToRefs(modeStore);

const { handleReEvaluate } = useReEvaluate();
const scrollSignal = ref(0);
watch([result, evaluateResult], () => scrollSignal.value++);
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

      <ScrollIntoView :trigger="scrollSignal">
        <section v-if="mode === 'optimize' && result" class="grid gap-6 lg:grid-cols-2">
          <MetricsBreakdown :metrics="result.metrics" />
          <CostHistory />
        </section>

        <section v-if="mode === 'evaluate' && evaluateResult && qwertyResult">
          <ComparisonChart :user-result="evaluateResult" :qwerty-result="qwertyResult" />
        </section>
      </ScrollIntoView>

      <Footer />
    </div>
  </main>
</template>
