<script setup lang="ts">
import { Badge } from '@/components/ui/badge';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { useField } from 'vee-validate';
import { computed, watch } from 'vue';

import { corpusTextControl as control } from '../controls';
import { useCorpusStore } from '../store';

const { value, errorMessage } = useField<string>(control.key);
const characterCount = computed(() => value.value?.trim().length ?? 0);

const corpusStore = useCorpusStore();
watch(
  value,
  (text) => {
    corpusStore.setText(text ?? '');
  },
  { immediate: true },
);
</script>

<template>
  <Card>
    <CardHeader class="flex flex-col gap-3 space-y-0 sm:flex-row sm:items-start sm:justify-between">
      <div>
        <CardTitle>Corpus</CardTitle>
        <CardDescription class="mt-1">
          {{ control.description }}
        </CardDescription>
      </div>
      <Badge variant="secondary">{{ characterCount }} characters</Badge>
    </CardHeader>

    <CardContent class="pt-5">
      <Label :for="control.key">{{ control.label }}</Label>
      <Textarea v-model="value" :id="control.key" rows="9" class="mt-2 resize-y" />
      <p v-if="errorMessage" class="mt-2 text-xs leading-5 text-destructive">
        {{ errorMessage }}
      </p>
    </CardContent>
  </Card>
</template>
