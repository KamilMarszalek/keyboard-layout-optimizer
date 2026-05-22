<script setup lang="ts">
import Header from '@/components/common/Header.vue';
import MetricWeights from '@/features/config/components/MetricWeights.vue';
import AnnealingParams from '@/features/config/components/AnnealingParams.vue';
import Corpus from '@/features/corpus/components/Corpus.vue';
import Seed from '@/features/config/components/Seed.vue';
</script>

<template>
  <Header />
  <Seed />
  <Corpus />
  <MetricWeights />
  <AnnealingParams />
</template>
