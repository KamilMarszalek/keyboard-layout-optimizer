<script setup lang="ts">
import { Alert } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { useResultsStore } from '@/features/results/store';
import { storeToRefs } from 'pinia';
import { computed, onMounted } from 'vue';

import { buildFrequencyMap, maxFrequency } from '../heatmap';
import { useKeyboardStore } from '../store';
import { EXPECTED_LAYOUT_LENGTH, KEYBOARD_ROW_OFFSETS, layoutToRows } from '../layout.ts';
import HeatmapToggle from './HeatmapToggle.vue';
import Row from './Row.vue';

const store = useKeyboardStore();
const resultsStore = useResultsStore();

const { standardQwertyLayout, isLoadingQwerty, layoutError, charFrequencies } = storeToRefs(store);
const { result } = storeToRefs(resultsStore);

const layoutMetadata = computed(() => {
  return result?.value?.bestLayout
    ? { layout: result.value.bestLayout, title: 'Optimized layout' }
    : { layout: standardQwertyLayout.value, title: 'Standard QWERTY layout' };
});

const displayedLayoutRows = computed(() => layoutToRows(layoutMetadata.value.layout));
const freqMap = computed(() => buildFrequencyMap(charFrequencies?.value));
const maxFreq = computed(() => maxFrequency(charFrequencies?.value));

onMounted(() => {
  void store.loadStandardQwertyLayout();
});
</script>

<template>
  <Card>
    <CardHeader class="flex flex-col gap-3 space-y-0 sm:flex-row sm:items-end sm:justify-between">
      <div>
        <CardTitle>{{ layoutMetadata.title }}</CardTitle>
        <CardDescription class="mt-1">
          Keys are shown in fixed physical ANSI positions.
        </CardDescription>
      </div>
      <Badge variant="secondary">{{ EXPECTED_LAYOUT_LENGTH }} keys</Badge>
    </CardHeader>

    <CardContent class="pt-5">
      <HeatmapToggle class="mb-5" />

      <Alert v-if="layoutError" variant="destructive" class="mb-5">
        {{ layoutError }}
      </Alert>

      <div
        v-if="isLoadingQwerty"
        class="rounded-md border border-dashed border-border p-6 text-sm text-muted-foreground"
      >
        Loading standard keyboard layout...
      </div>

      <div v-else class="overflow-x-auto pb-2">
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
