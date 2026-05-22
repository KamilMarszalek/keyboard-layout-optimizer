<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { useCorpusStore } from '../corpus.store';

const store = useCorpusStore();
const { text } = storeToRefs(store);
</script>

<template>
  <Card>
    <CardHeader class="flex flex-col gap-3 space-y-0 sm:flex-row sm:items-start sm:justify-between">
      <div>
        <CardTitle>Corpus</CardTitle>
        <CardDescription class="mt-1">
          Paste representative text for the optimizer to evaluate.
        </CardDescription>
      </div>
      <Badge variant="secondary"> {{ store.characterCount }} characters </Badge>
    </CardHeader>

    <CardContent class="pt-5">
      <Label for="corpus-text">Input text</Label>
      <Textarea v-model="text" id="corpus-text" rows="9" class="mt-2 resize-y" />
    </CardContent>
  </Card>
</template>
