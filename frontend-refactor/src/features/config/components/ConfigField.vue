<script setup lang="ts">
import { Label } from '@/components/ui/label';

defineProps<{
  id: string;
  label: string;
  description?: string;
  error?: string;
}>();
</script>

<template>
  <div class="space-y-2">
    <div class="flex items-start justify-between gap-4">
      <div>
        <Label :for="id">{{ label }}</Label>
        <p v-if="description" class="mt-1 text-xs leading-5 text-muted-foreground">
          {{ description }}
        </p>
      </div>
      <slot name="value" />
    </div>
    <slot />
    <p v-if="error" class="text-xs leading-5 text-destructive">
      {{ error }}
    </p>
  </div>
</template>
