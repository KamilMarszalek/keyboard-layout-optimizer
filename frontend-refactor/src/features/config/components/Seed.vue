<script setup lang="ts">
import { computed } from 'vue';
import { storeToRefs } from 'pinia';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { useConfigStore } from '../config.store';

const store = useConfigStore();
const { seed } = storeToRefs(store);
const seedInput = computed({
  get: () => seed.value ?? '',
  set: (val: string) => (seed.value = val === '' ? null : Number(val)),
});
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Seed</CardTitle>
    </CardHeader>
    <CardContent class="pt-5">
      <Label for="seed">Optional seed</Label>
      <Input
        v-model="seedInput"
        id="seed"
        type="number"
        class="mt-2"
        placeholder="Leave empty for no seed"
      />
    </CardContent>
  </Card>
</template>
