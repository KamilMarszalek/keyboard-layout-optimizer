<script setup lang="ts">
import { toTypedSchema } from '@vee-validate/zod';
import { useForm } from 'vee-validate';
import {
  AnnealingParams,
  MetricWeights,
  Seed,
} from '@/features/config/components';
import { defaultConfig } from '@/features/config/config.schema';
import { Corpus } from '@/features/corpus/components';
import { defaultCorpus } from '@/features/corpus/corpus.schema';
import { optimizeRequestSchema } from '../optimizer.schema';
import { useOptimizerStore } from '../optimizer.store';
import Run from './Run.vue';

const optimizer = useOptimizerStore();

const { handleSubmit } = useForm({
  validationSchema: toTypedSchema(optimizeRequestSchema),
  initialValues: {
    config: defaultConfig,
    corpus: defaultCorpus,
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
