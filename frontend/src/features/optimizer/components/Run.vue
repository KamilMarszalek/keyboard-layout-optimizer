<script setup lang="ts">
import { Alert } from '@/components/ui/alert';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { storeToRefs } from 'pinia';

import { useOptimizerStore } from '../optimizer.store';

const store = useOptimizerStore();
const { error, isOptimizing } = storeToRefs(store);
</script>

<template>
  <Card class="flex flex-col">
    <CardHeader>
      <CardTitle>Run</CardTitle>
      <CardDescription>
        The optimizer compares QWERTY and Dvorak starts, then returns the best result.
      </CardDescription>
    </CardHeader>

    <CardContent class="flex flex-1 flex-col pt-5">
      <Alert v-if="error" variant="destructive" class="mb-4">
        {{ error }}
      </Alert>

      <div class="mt-auto space-y-3">
        <Button type="submit" size="lg" class="w-full" :disabled="isOptimizing">
          {{ isOptimizing ? 'Optimizing...' : 'Optimize layout' }}
        </Button>
      </div>
    </CardContent>
  </Card>
</template>
