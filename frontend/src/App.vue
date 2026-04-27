<script setup lang="ts">
import { onMounted, ref } from "vue";
import init, { normalize_demo, optimizer_demo_message } from "./wasm/optimizer";
import { standardKeyboardRows } from "./keyboard";

const input = ref("Ala ma Kota! 123");
const normalized = ref("");
const wasmMessage = ref("WASM module not loaded yet");
const rows = standardKeyboardRows();

async function runWasmDemo() {
  await init();

  wasmMessage.value = optimizer_demo_message();
  normalized.value = normalize_demo(input.value);
}

onMounted(() => {
  void runWasmDemo();
});
</script>

<template>
  <main>
    <h1>Keyboard Layout Optimizer</h1>

    <p>{{ wasmMessage }}</p>

    <label for="input-text">Input text</label>
    <textarea id="input-text" v-model="input"></textarea>

    <button type="button" @click="runWasmDemo">
      Normalize with WASM
    </button>

    <section>
      <h2>WASM normalized text demo</h2>
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