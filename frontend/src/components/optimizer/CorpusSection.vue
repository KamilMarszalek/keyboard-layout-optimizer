<script setup lang="ts">
import { storeToRefs } from "pinia";
import { Badge } from "../ui/badge";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "../ui/card";
import { Label } from "../ui/label";
import { Textarea } from "../ui/textarea";
import { useOptimizerStore } from "../../stores/optimizerStore";

const store = useOptimizerStore();
const { corpusCharacterCount, corpusText } = storeToRefs(store);
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
      <Badge variant="secondary">
        {{ corpusCharacterCount }} characters
      </Badge>
    </CardHeader>

    <CardContent class="pt-5">
      <Label for="corpus-text">Input text</Label>
      <Textarea
        id="corpus-text"
        v-model="corpusText"
        rows="9"
        class="mt-2 resize-y"
      />
    </CardContent>
  </Card>
</template>
