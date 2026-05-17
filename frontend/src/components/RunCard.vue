<script setup lang="ts">
defineProps<{
  error: string | null;
  isOptimizing: boolean;
}>();

defineEmits<{
  optimize: [];
}>();

const seed = defineModel<number | "">("seed", { required: true });
</script>

<template>
  <section class="rounded-xl bg-white p-5 shadow-sm ring-1 ring-slate-200 sm:p-6">
    <h2 class="text-xl font-semibold text-slate-950">
      Run
    </h2>
    <p class="mt-1 text-sm text-slate-600">
      The optimizer compares QWERTY and Dvorak starts, then returns the best result.
    </p>

    <label for="seed" class="mt-5 block text-sm font-medium text-slate-700">
      Optional seed
    </label>
    <input
      id="seed"
      v-model.number="seed"
      type="number"
      class="mt-2 block w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 shadow-sm outline-none transition focus:border-teal-500 focus:ring-4 focus:ring-teal-100"
      placeholder="Leave empty for no seed"
    />

    <button
      type="button"
      :disabled="isOptimizing"
      class="mt-6 inline-flex w-full items-center justify-center rounded-lg bg-teal-700 px-4 py-3 text-sm font-semibold text-white shadow-sm transition hover:bg-teal-800 focus:outline-none focus:ring-4 focus:ring-teal-200 disabled:cursor-not-allowed disabled:bg-slate-400 disabled:shadow-none"
      @click="$emit('optimize')"
    >
      {{ isOptimizing ? "Optimizing..." : "Optimize layout" }}
    </button>

    <p
      v-if="error"
      class="mt-4 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700"
    >
      {{ error }}
    </p>
  </section>
</template>
