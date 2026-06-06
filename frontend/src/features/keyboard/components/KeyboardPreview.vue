<script setup lang="ts">
import { Alert } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { useResultsStore } from '@/features/results/store';
import { storeToRefs } from 'pinia';
import { computed, onMounted } from 'vue';

import { buildFrequencyMap, keyHeatStyle, maxFrequency } from '../heatmap';
import { EXPECTED_LAYOUT_LENGTH, KEYBOARD_ROW_OFFSETS, layoutToRows } from '../layout.ts';
import { useKeyboardStore } from '../store';
import { useKeyDragAndDrop } from '../useKeyDragAndDrop';
import HeatmapToggle from './HeatmapToggle.vue';
import Row from './Row.vue';

const props = withDefaults(defineProps<{ editable?: boolean }>(), {
  editable: false,
});

const store = useKeyboardStore();
const resultsStore = useResultsStore();

const {
  standardQwertyLayout,
  editableLayout,
  isLoadingQwerty,
  layoutError,
  charFrequencies,
  showHeatmap,
} = storeToRefs(store);
const { result } = storeToRefs(resultsStore);

const { onDragStart, onDragOver, onDrop } = useKeyDragAndDrop();

const layoutMetadata = computed(() => {
  if (props.editable) {
    return { layout: editableLayout.value, title: 'Edit layout' };
  }
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

const rowStartIndices = computed(() => {
  const starts: number[] = [];
  let offset = 0;
  for (const row of rows.value) {
    starts.push(offset);
    offset += row.length;
  }
  return starts;
});

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
      <div class="flex items-center gap-3">
        <Button
          v-if="editable"
          type="button"
          variant="outline"
          size="sm"
          @click="store.resetEditableLayout()"
        >
          Reset to QWERTY
        </Button>
        <Badge variant="secondary">{{ EXPECTED_LAYOUT_LENGTH }} keys</Badge>
      </div>
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
            :editable="editable"
            :start-index="rowStartIndices[rowIndex]"
            @dragstart="onDragStart"
            @dragover="onDragOver"
            @drop="onDrop"
          />
        </div>
      </div>
    </CardContent>
  </Card>
</template>
