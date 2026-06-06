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
  <Card>
    <CardHeader>
      <CardTitle>Run</CardTitle>
      <CardDescription class="mt-1">
        The optimizer compares QWERTY and Dvorak starts, then returns the best result.
      </CardDescription>
    </CardHeader>

    <CardContent class="pt-5">
      <Button type="submit" size="lg" class="w-full" :disabled="isOptimizing">
        {{ isOptimizing ? 'Optimizing...' : 'Optimize layout' }}
      </Button>

      <Alert v-if="error" variant="destructive" class="mt-4">
        {{ error }}
      </Alert>
    </CardContent>
  </Card>
</template>
