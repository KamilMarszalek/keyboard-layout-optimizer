<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { useDark, useToggle } from '@vueuse/core';
import { Moon, Sun } from 'lucide-vue-next';

const isDark = useDark();
const toggleDark = useToggle(isDark);
</script>

<template>
  <header class="space-y-2">
    <div class="flex items-start justify-between gap-4">
      <p class="text-sm font-semibold uppercase tracking-wide text-foreground">
        Keyboard layout optimizer
      </p>
      <Button variant="ghost" size="icon" @click="toggleDark()">
        <Moon v-if="isDark" class="size-5" />
        <Sun v-else class="size-5" />
      </Button>
    </div>
    <h1 class="text-3xl font-semibold tracking-normal text-foreground sm:text-4xl">
      Optimize and Evaluate Keyboard Layouts
    </h1>
    <p class="text-base leading-7 text-muted-foreground">
      Tune metric weights and simulated annealing parameters, then run the Rust/WASM optimizer
      directly in the browser &mdash; or evaluate and compare custom layout against QWERTY baseline.
    </p>
  </header>
</template>
