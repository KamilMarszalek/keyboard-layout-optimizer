<script setup lang="ts">
import { Alert } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { useResultsStore } from '@/features/results/store';
import { storeToRefs } from 'pinia';
import { computed, onMounted } from 'vue';

import { buildFrequencyMap, keyHeatStyle, maxFrequency } from '../heatmap';
import { EXPECTED_LAYOUT_LENGTH, KEYBOARD_ROW_OFFSETS, layoutToRows } from '../layout.ts';
import { useKeyboardStore } from '../store';
import HeatmapToggle from './HeatmapToggle.vue';
import Row from './Row.vue';

const store = useKeyboardStore();
const resultsStore = useResultsStore();

const { standardQwertyLayout, isLoadingQwerty, layoutError, charFrequencies, showHeatmap } =
  storeToRefs(store);
const { result } = storeToRefs(resultsStore);

const layoutMetadata = computed(() => {
  return result?.value?.bestLayout
    ? { layout: result.value.bestLayout, title: 'Optimized layout' }
    : { layout: standardQwertyLayout.value, title: 'Standard QWERTY layout' };
});

const freqMap = computed(() => buildFrequencyMap(charFrequencies?.value));
const maxFreq = computed(() => maxFrequency(charFrequencies?.value));

const rows = computed(() =>
  layoutToRows(layoutMetadata.value.layout).map((row) =>
    row.map((key) => {
      const freq = freqMap.value.get(key.base) ?? 0;
      return {
        mapping: key,
        style:
          showHeatmap.value && freq !== 0
            ? keyHeatStyle(freq, maxFreq.value)
            : { backgroundColor: 'transparent' },
      };
    }),
  ),
);

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
            v-for="(rowView, rowIndex) in rows"
            :key="`layout-row-${rowIndex}`"
            :views="rowView"
            :offset-class="KEYBOARD_ROW_OFFSETS[rowIndex]"
          />
        </div>
      </div>
    </CardContent>
  </Card>
</template>
