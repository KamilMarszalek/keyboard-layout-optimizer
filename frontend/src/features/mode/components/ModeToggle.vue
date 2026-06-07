<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { useOptimizerStore } from '@/features/optimizer/store';
import { storeToRefs } from 'pinia';

import { useModeStore } from '../store';

const modeStore = useModeStore();
const { mode } = storeToRefs(modeStore);

const optimizerStore = useOptimizerStore();
const { isOptimizing } = storeToRefs(optimizerStore);
</script>

<template>
  <div class="flex justify-center">
    <div class="inline-flex gap-1 rounded-md border border-border bg-muted/40 p-1">
      <Button
        type="button"
        size="sm"
        :variant="mode === 'optimize' ? 'default' : 'ghost'"
        @click="modeStore.setMode('optimize')"
      >
        Optimize
      </Button>
      <div :class="isOptimizing ? 'opacity-50 pointer-events-none select-none' : ''">
        <Button
          type="button"
          size="sm"
          :variant="mode === 'evaluate' ? 'default' : 'ghost'"
          @click="modeStore.setMode('evaluate')"
        >
          Evaluate
        </Button>
      </div>
    </div>
  </div>
</template>
