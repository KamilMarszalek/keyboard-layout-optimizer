<script setup lang="ts">
import { storeToRefs } from "pinia";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "../ui/card";
import { Input } from "../ui/input";
import { Label } from "../ui/label";
import { annealingFields } from "../../constants/optimizerControls";
import { useOptimizerStore } from "../../stores/optimizerStore";

const store = useOptimizerStore();
const { annealing } = storeToRefs(store);
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Annealing parameters</CardTitle>
      <CardDescription class="mt-1">
        Control the cooling schedule and work performed at each temperature. Higher iteration counts
        can take longer in the browser.
      </CardDescription>
    </CardHeader>

    <CardContent class="grid gap-4 pt-5 sm:grid-cols-2 lg:grid-cols-4">
      <div
        v-for="field in annealingFields"
        :key="field.key"
      >
        <Label :for="`annealing-${field.key}`">
          {{ field.label }}
        </Label>
        <Input
          :id="`annealing-${field.key}`"
          v-model.number="annealing[field.key]"
          type="number"
          :step="field.step"
          :min="field.min"
          :max="field.max"
          class="mt-2"
        />
      </div>
    </CardContent>
  </Card>
</template>
