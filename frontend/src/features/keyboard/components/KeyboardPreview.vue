<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { computed } from 'vue';
import { Alert } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import {
  EXPECTED_LAYOUT_LENGTH,
  KEYBOARD_ROW_OFFSETS,
  hasExpectedLayoutLength,
  layoutToRows,
} from '../keyboardLayout';
import type { CharFrequencyDto } from '@/services/optimizer/optimizer.dto';
import { useKeyboardStore } from '../keyboard.store';

const props = defineProps<{
  optimizedLayout?: string[];
  charFrequencies?: CharFrequencyDto[];
}>();

const store = useKeyboardStore();
const { standardQwertyLayout, isLoadingQwerty, layoutError } = storeToRefs(store);

const currentLayout = computed(() => {
  if (props.optimizedLayout) {
    return hasExpectedLayoutLength(props.optimizedLayout) ? props.optimizedLayout : [];
  }
  return hasExpectedLayoutLength(standardQwertyLayout.value) ? standardQwertyLayout.value : [];
});

const layoutTitle = computed(() =>
  props.optimizedLayout ? 'Optimized layout' : 'Standard QWERTY layout',
);

const layoutValidationMessage = computed((): string | null => {
  if (layoutError.value) {
    return layoutError.value;
  }

  if (props.optimizedLayout && !hasExpectedLayoutLength(props.optimizedLayout)) {
    return `The optimizer returned ${props.optimizedLayout.length} keys; expected ${EXPECTED_LAYOUT_LENGTH}.`;
  }

  if (
    !props.optimizedLayout &&
    standardQwertyLayout.value.length > 0 &&
    !hasExpectedLayoutLength(standardQwertyLayout.value)
  ) {
    return `The standard keyboard layout loaded ${standardQwertyLayout.value.length} keys; expected ${EXPECTED_LAYOUT_LENGTH}.`;
  }

  return null;
});

const displayedLayoutRows = computed(() => layoutToRows(currentLayout.value));

const freqMap = computed((): Map<string, number> => {
  if (!props.charFrequencies) return new Map();
  return new Map(props.charFrequencies.map(({ key, frequency }) => [key, frequency]));
});

const maxFreq = computed((): number => {
  if (!props.charFrequencies || props.charFrequencies.length === 0) return 1;
  return Math.max(...props.charFrequencies.map((f) => f.frequency));
});

function keyHeatStyle(keyLabel: string): { backgroundColor: string } | undefined {
  if (!props.charFrequencies) return undefined;

  const freq = freqMap.value.get(keyLabel) ?? 0;
  const ratio = maxFreq.value > 0 ? freq / maxFreq.value : 0;

  // Interpolate from light neutral (low/no freq) to amber (high freq)
  const hue = 30;
  const saturation = Math.round(15 + ratio * 75); // 15 % → 90 %
  const lightness = Math.round(88 - ratio * 36);  // 88 % → 52 %

  return { backgroundColor: `hsl(${hue}, ${saturation}%, ${lightness}%)` };
}
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

      <div v-else-if="currentLayout.length > 0" class="overflow-x-auto pb-2">
        <div class="mx-auto w-max space-y-2 rounded-md bg-muted/40 p-4 ring-1 ring-border">
          <div
            v-for="(row, rowIndex) in displayedLayoutRows"
            :key="`layout-row-${rowIndex}`"
            class="flex gap-2"
            :class="KEYBOARD_ROW_OFFSETS[rowIndex]"
          >
            <div
              v-for="(keyLabel, keyIndex) in row"
              :key="`${rowIndex}-${keyIndex}-${keyLabel}`"
              class="flex h-12 w-12 shrink-0 items-center justify-center rounded-md border border-border bg-card font-mono text-base font-semibold text-card-foreground shadow-sm"
              :style="keyHeatStyle(keyLabel)"
            >
              {{ keyLabel }}
            </div>
          </div>
        </div>
      </div>
    </CardContent>
  </Card>
</template>
