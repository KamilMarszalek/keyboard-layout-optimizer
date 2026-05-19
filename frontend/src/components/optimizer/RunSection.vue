<script setup lang="ts">
import { storeToRefs } from "pinia";
import { Alert } from "../ui/alert";
import { Button } from "../ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "../ui/card";
import { Input } from "../ui/input";
import { Label } from "../ui/label";
import { useOptimizerStore } from "../../stores/optimizerStore";

const store = useOptimizerStore();
const { displayedError, isOptimizing, seed } = storeToRefs(store);
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Run</CardTitle>
      <CardDescription class="mt-1">
        The optimizer compares QWERTY and Dvorak starts, then returns the best result.
      </CardDescription>
    </CardHeader>

    <CardContent class="pt-5">
      <Label for="seed">Optional seed</Label>
      <Input
        id="seed"
        v-model.number="seed"
        type="number"
        class="mt-2"
        placeholder="Leave empty for no seed"
      />

      <Button
        type="button"
        size="lg"
        class="mt-6 w-full"
        :disabled="isOptimizing"
        @click="store.runOptimization()"
      >
        {{ isOptimizing ? "Optimizing..." : "Optimize layout" }}
      </Button>

      <Alert
        v-if="displayedError"
        variant="destructive"
        class="mt-4"
      >
        {{ displayedError }}
      </Alert>
    </CardContent>
  </Card>
</template>
