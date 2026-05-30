<script setup lang="ts">
import { useField } from 'vee-validate';
import { computed, watch } from 'vue';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { corpusTextControl as control } from '../corpus.controls';
import { useKeyboardStore } from '@/features/keyboard/keyboard.store';

const { value, errorMessage } = useField<string>(`corpus.${control.key}`);
const characterCount = computed(() => value.value?.trim().length ?? 0);

const keyboardStore = useKeyboardStore();
watch(value, (text) => { keyboardStore.setCorpusText(text ?? ''); }, { immediate: true });
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
