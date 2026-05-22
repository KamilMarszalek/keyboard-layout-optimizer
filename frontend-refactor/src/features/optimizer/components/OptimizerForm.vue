<script setup lang="ts">
import { toTypedSchema } from '@vee-validate/zod';
import { useForm } from 'vee-validate';
import AnnealingParams from '@/features/config/components/AnnealingParams.vue';
import MetricWeights from '@/features/config/components/MetricWeights.vue';
import Seed from '@/features/config/components/Seed.vue';
import { defaultConfig } from '@/features/config/config.schema';
import Corpus from '@/features/corpus/components/Corpus.vue';
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
    <MetricWeights />
    <AnnealingParams />
    <Seed />
    <Run />
  </form>
</template>
