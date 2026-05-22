<script setup lang="ts">
import Header from '@/components/common/Header.vue';
import { OptimizerForm } from '@/features/optimizer/components';
import { useResultsStore } from '@/features/results/results.store';
import { storeToRefs } from 'pinia';
import { OptimizationResult } from '@/features/results/components';

const store = useResultsStore();
const { result } = storeToRefs(store);
</script>

<template>
  <main class="min-h-screen px-4 py-8 sm:px-6 lg:px-8">
    <div class="mx-auto max-w-7xl space-y-6">
      <Header />
      <OptimizerForm />
    </div>
    <section v-if="result">
      <OptimizationResult />
      <div class="space-y-6">
        <MetricsBreakdown />
        <CostHistoryList />
      </div>
    </section>
  </main>
</template>
