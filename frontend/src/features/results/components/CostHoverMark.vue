<script setup lang="ts">
import { HoverCard, HoverCardContent, HoverCardTrigger } from '@/components/ui/hover-card';
import { ref, watch } from 'vue';

const props = defineProps<{
  value: number;
}>();

const hasUnseenChange = ref(false);
const onOpen = (open: boolean) => {
  if (open) {
    hasUnseenChange.value = false;
  }
};

watch(
  () => props.value,
  () => {
    hasUnseenChange.value = true;
  },
);
</script>

<template>
  <HoverCard :open-delay="100" :close-delay="100" @update:open="onOpen">
    <HoverCardTrigger
      as="button"
      type="button"
      class="relative flex h-6 w-6 items-center justify-center rounded-full border border-border text-base font-bold text-muted-foreground hover:bg-muted"
    >
      ?
      <span
        v-if="hasUnseenChange"
        class="absolute -right-0.5 -top-0.5 h-2 w-2 rounded-full bg-destructive ring-2 ring-card"
      />
    </HoverCardTrigger>
    <HoverCardContent align="end" class="w-72">
      <slot />
    </HoverCardContent>
  </HoverCard>
</template>
