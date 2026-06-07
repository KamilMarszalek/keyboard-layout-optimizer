<script setup lang="ts">
import { AnnealingParams, MetricWeights, Seed } from '@/features/config/components';
import {
  type MetricsWeights,
  defaultAnnealingParams,
  defaultSeed,
  defaultWeights,
} from '@/features/config/schema';
import { useWeightsStore } from '@/features/config/store.ts';
import { Corpus } from '@/features/corpus/components';
import { defaultText } from '@/features/corpus/schema';
import { Evaluate } from '@/features/evaluator/components';
import { ModeToggle } from '@/features/mode/components';
import { useModeStore } from '@/features/mode/store';
import { toTypedSchema } from '@vee-validate/zod';
import { storeToRefs } from 'pinia';
import { useForm } from 'vee-validate';
import { onBeforeUnmount, watch } from 'vue';

import { optimizeRequestSchema } from '../schema.ts';
import { useOptimizerStore } from '../store.ts';
import Run from './Run.vue';

const optimizer = useOptimizerStore();
const weightsStore = useWeightsStore();
const modeStore = useModeStore();
const { mode } = storeToRefs(modeStore);

const { handleSubmit, values } = useForm({
  validationSchema: toTypedSchema(optimizeRequestSchema),
  initialValues: {
    weights: defaultWeights,
    annealing: defaultAnnealingParams,
    seed: defaultSeed,
    text: defaultText,
  },
});

watch(
  () => values.weights,
  (weights) => {
    if (weights) {
      weightsStore.setWeights(weights as MetricsWeights);
    }
  },
  { immediate: true, deep: true },
);

const onSubmit = handleSubmit((values) => optimizer.run(values));

onBeforeUnmount(() => optimizer.dispose());
</script>

<template>
  <form class="space-y-6 novalidate" @submit="onSubmit">
    <Corpus />
    <ModeToggle />
    <div class="grid gap-6 lg:grid-cols-2">
      <div
        :class="[
          'h-full [&>*]:h-full',
          mode === 'evaluate' ? 'opacity-50 pointer-events-none select-none' : '',
        ]"
      >
        <AnnealingParams />
      </div>
      <MetricWeights />
    </div>
    <div class="grid gap-6 lg:grid-cols-2">
      <div
        :class="[
          '[&>*]:h-full',
          mode === 'evaluate' ? 'opacity-50 pointer-events-none select-none' : '',
        ]"
      >
        <Seed />
      </div>
      <Run v-if="mode === 'optimize'" />
      <Evaluate v-else />
    </div>
  </form>
</template>
