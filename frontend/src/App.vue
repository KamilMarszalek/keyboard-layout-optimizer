<script setup lang="ts">
import Header from '@/components/common/Header.vue';
import { KeyboardPreview } from '@/features/keyboard/components';
import { OptimizerForm } from '@/features/optimizer/components';
import { CostHistory, MetricsBreakdown, OptimizationResult } from '@/features/results/components';
import { useResultsStore } from '@/features/results/store';
import { storeToRefs } from 'pinia';

const resultStore = useResultsStore();
const { result } = storeToRefs(resultStore);
</script>

<template>
  <main class="min-h-screen px-4 py-8 sm:px-6 lg:px-8">
    <div class="mx-auto max-w-7xl space-y-6">
      <Header />
      <OptimizerForm />
      <KeyboardPreview />
      <section v-if="result" class="grid gap-6 lg:grid-cols-[minmax(0,1fr)_minmax(360px,0.8fr)]">
        <OptimizationResult />
        <div class="space-y-6">
          <MetricsBreakdown />
          <CostHistory />
        </div>
      </section>
    </div>
  </main>
</template>
