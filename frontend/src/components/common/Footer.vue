<script setup lang="ts">
import githubIcon from '@/assets/github.svg?raw';
</script>

<template>
  <footer
    class="flex flex-col items-center justify-between gap-2 border-t border-border pt-6 text-sm text-muted-foreground sm:flex-row"
  >
    <p>
      Made by
      <span class="font-medium text-foreground">Kamil Marszałek & Michał Szwejk</span>
    </p>
    <a
      href="https://github.com/KamilMarszalek/keyboard-layout-optimizer"
      target="_blank"
      rel="noopener noreferrer"
      class="inline-flex items-center gap-1.5 transition-colors hover:text-foreground"
    >
      <span class="[&>svg]:size-4" v-html="githubIcon" />
      Github
    </a>
  </footer>
</template>
