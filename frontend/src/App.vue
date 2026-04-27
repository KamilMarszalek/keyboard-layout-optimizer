<script setup lang="ts">
import { computed, ref } from "vue";
import { normalizeText, standardKeyboardRows } from "./keyboard";

const input = ref("Ala ma Kota! 123");
const normalized = computed(() => normalizeText(input.value));
const rows = standardKeyboardRows();
</script>

<template>
  <main>
    <h1>Keyboard Layout Optimizer</h1>

    <label for="input-text">Input text</label>
    <textarea id="input-text" v-model="input"></textarea>

    <section>
      <h2>Normalized text demo</h2>
      <pre>{{ normalized }}</pre>
    </section>

    <section>
      <h2>Keyboard preview</h2>
      <div v-for="(row, index) in rows" :key="index">
        <button v-for="key in row" :key="key">
          {{ key }}
        </button>
      </div>
    </section>
  </main>
</template>