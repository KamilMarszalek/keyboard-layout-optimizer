<script setup lang="ts">
import { AnnealingParams, MetricWeights, Seed } from '@/features/config/components';
import {
  defaultAnnealingParams,
  defaultSeed,
  defaultWeights,
} from '@/features/config/config.schema';
import { Corpus } from '@/features/corpus/components';
import { defaultText } from '@/features/corpus/corpus.schema';
import { toTypedSchema } from '@vee-validate/zod';
import { useForm } from 'vee-validate';

import { optimizeRequestSchema } from '../optimizer.schema.ts';
import { useOptimizerStore } from '../optimizer.store.ts';
import Run from './Run.vue';

const optimizer = useOptimizerStore();

const { handleSubmit } = useForm({
  validationSchema: toTypedSchema(optimizeRequestSchema),
  initialValues: {
    weights: defaultWeights,
    annealing: defaultAnnealingParams,
    seed: defaultSeed,
    text: defaultText,
  },
});

const onSubmit = handleSubmit((values) => optimizer.run(values));
</script>

<template>
  <form class="space-y-6 novalidate" @submit="onSubmit">
    <Corpus />
    <div class="grid gap-6 lg:grid-cols-2">
      <MetricWeights />
      <AnnealingParams />
    </div>
    <div class="grid gap-6 lg:grid-cols-2">
      <Seed />
      <Run />
    </div>
  </form>
</template>
