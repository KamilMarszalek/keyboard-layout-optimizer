<script setup lang="ts">
import { Alert } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { storeToRefs } from 'pinia';
import { computed } from 'vue';

import { buildFrequencyMap, maxFrequency } from '../heatmap';
import { useKeyboardStore } from '../keyboard.store';
import {
  EXPECTED_LAYOUT_LENGTH,
  KEYBOARD_ROW_OFFSETS,
  hasExpectedLayoutLength,
  layoutToRows,
} from '../keyboardLayout';
import type { CharFrequency, Layout } from '../types';
import Row from './Row.vue';

const EMPTY_LAYOUT: Layout = { mappings: [] };

const props = defineProps<{
  optimizedLayout?: Layout;
  charFrequencies?: CharFrequency[];
}>();

const store = useKeyboardStore();
const { standardQwertyLayout, isLoadingQwerty, layoutError } = storeToRefs(store);

const currentLayout = computed((): Layout => {
  const layout = props.optimizedLayout ?? standardQwertyLayout.value;
  return hasExpectedLayoutLength(layout) ? layout : EMPTY_LAYOUT;
});

const layoutTitle = computed(() =>
  props.optimizedLayout ? 'Optimized layout' : 'Standard QWERTY layout',
);

const layoutValidationMessage = computed((): string | null => {
  if (layoutError.value) {
    return layoutError.value;
  }

  if (props.optimizedLayout && !hasExpectedLayoutLength(props.optimizedLayout)) {
    return `The optimizer returned ${props.optimizedLayout.mappings.length} keys; expected ${EXPECTED_LAYOUT_LENGTH}.`;
  }

  if (
    !props.optimizedLayout &&
    standardQwertyLayout.value.mappings.length > 0 &&
    !hasExpectedLayoutLength(standardQwertyLayout.value)
  ) {
    return `The standard keyboard layout loaded ${standardQwertyLayout.value.mappings.length} keys; expected ${EXPECTED_LAYOUT_LENGTH}.`;
  }

  return null;
});

const displayedLayoutRows = computed(() => layoutToRows(currentLayout.value));

const freqMap = computed(() => buildFrequencyMap(props.charFrequencies));
const maxFreq = computed(() => maxFrequency(props.charFrequencies));
</script>

<template>
  <Card>
    <CardHeader class="flex flex-col gap-3 space-y-0 sm:flex-row sm:items-end sm:justify-between">
      <div>
        <CardTitle>{{ layoutTitle }}</CardTitle>
        <CardDescription class="mt-1">
          Keys are shown in fixed physical ANSI positions.
        </CardDescription>
      </div>
      <Badge variant="secondary">{{ EXPECTED_LAYOUT_LENGTH }} keys</Badge>
    </CardHeader>

    <CardContent class="pt-5">
      <Alert v-if="layoutValidationMessage" variant="destructive" class="mb-5">
        {{ layoutValidationMessage }}
      </Alert>

      <div
        v-if="isLoadingQwerty"
        class="rounded-md border border-dashed border-border p-6 text-sm text-muted-foreground"
      >
        Loading standard keyboard layout...
      </div>

      <div v-else-if="currentLayout.mappings.length > 0" class="overflow-x-auto pb-2">
        <div class="mx-auto w-max space-y-2 rounded-md bg-muted/40 p-4 ring-1 ring-border">
          <Row
            v-for="(row, rowIndex) in displayedLayoutRows"
            :key="`layout-row-${rowIndex}`"
            :mappings="row"
            :offset-class="KEYBOARD_ROW_OFFSETS[rowIndex]"
            :freq-map="freqMap"
            :max-freq="maxFreq"
          />
        </div>
      </div>
    </CardContent>
  </Card>
</template>
