<script setup lang="ts">
import { storeToRefs } from "pinia";
import { computed } from "vue";
import { Alert } from "../ui/alert";
import { Badge } from "../ui/badge";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "../ui/card";
import { KEYBOARD_ROW_OFFSETS, layoutToRows } from "../../lib/keyboardLayout";
import { useLayoutStore } from "../../stores/layoutStore";

const store = useLayoutStore();
const { currentLayout, expectedLayoutLength, layoutTitle, layoutValidationMessage } =
  storeToRefs(store);

const displayedLayoutRows = computed(() => layoutToRows(currentLayout.value));
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
      <Badge variant="secondary">
        {{ expectedLayoutLength }} keys
      </Badge>
    </CardHeader>

    <CardContent class="pt-5">
      <Alert
        v-if="layoutValidationMessage"
        variant="warning"
        class="mb-5"
      >
        {{ layoutValidationMessage }}
      </Alert>

      <div class="overflow-x-auto pb-2">
        <div class="stat-row mx-auto w-max space-y-2 p-4">
          <div
            v-for="(row, rowIndex) in displayedLayoutRows"
            :key="`layout-row-${rowIndex}`"
            class="flex gap-2"
            :class="KEYBOARD_ROW_OFFSETS[rowIndex]"
          >
            <div
              v-for="(keyLabel, keyIndex) in row"
              :key="`${rowIndex}-${keyIndex}-${keyLabel}`"
              class="flex h-12 w-12 shrink-0 items-center justify-center rounded-lg border border-slate-300 bg-white font-mono text-base font-semibold text-slate-900 shadow-sm"
            >
              {{ keyLabel }}
            </div>
          </div>
        </div>
      </div>
    </CardContent>
  </Card>
</template>
