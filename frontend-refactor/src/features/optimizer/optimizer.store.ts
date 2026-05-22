import { defineStore } from 'pinia';
import type { OptimizeRequest } from './optimizer.schema';

export const useOptimizerStore = defineStore('optimizer', {
  state: () => ({
    isOptimizing: false,
    error: null as string | null,
  }),
  actions: {
    async run(request: OptimizeRequest) {
      this.isOptimizing = true;
      this.error = null;
      try {
        // TODO: send `request` to the WASM optimizer worker.
        void request;
        await new Promise((resolve) => setTimeout(resolve, 1000));
      } catch (e) {
        this.error = e instanceof Error ? e.message : String(e);
      } finally {
        this.isOptimizing = false;
      }
    },
  },
});
