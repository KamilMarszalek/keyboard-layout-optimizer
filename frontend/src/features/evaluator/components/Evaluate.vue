<script setup lang="ts">
import { Alert } from '@/components/ui/alert';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { storeToRefs } from 'pinia';

import { useEvaluatorStore } from '../store';

const store = useEvaluatorStore();
const { error, isEvaluating } = storeToRefs(store);
</script>

<template>
  <Card class="flex flex-col">
    <CardHeader>
      <CardTitle>Evaluate</CardTitle>
      <CardDescription>
        Score the current layout against the corpus using the selected metric weights.
      </CardDescription>
    </CardHeader>

    <CardContent class="flex flex-1 flex-col pt-5">
      <Alert v-if="error" variant="destructive" class="mb-4">
        {{ error }}
      </Alert>

      <div class="mt-auto space-y-3">
        <Button type="submit" size="lg" class="w-full" :disabled="isEvaluating">
          {{ isEvaluating ? 'Evaluating...' : 'Evaluate layout' }}
        </Button>
      </div>
    </CardContent>
  </Card>
</template>
