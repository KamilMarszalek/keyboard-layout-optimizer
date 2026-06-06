<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { storeToRefs } from 'pinia';

import { useModeStore } from '../store';

const store = useModeStore();
const { mode } = storeToRefs(store);
</script>

<template>
  <div class="flex justify-center">
    <div class="inline-flex gap-1 rounded-md border border-border bg-muted/40 p-1">
      <Button
        type="button"
        size="sm"
        :variant="mode === 'optimize' ? 'default' : 'ghost'"
        @click="store.setMode('optimize')"
      >
        Optimize
      </Button>
      <Button
        type="button"
        size="sm"
        :variant="mode === 'evaluate' ? 'default' : 'ghost'"
        @click="store.setMode('evaluate')"
      >
        Evaluate
      </Button>
    </div>
  </div>
</template>
