import { defineStore } from 'pinia';

export type Mode = 'optimize' | 'evaluate';

interface ModeState {
  mode: Mode;
}

export const useModeStore = defineStore('mode', {
  state: (): ModeState => ({
    mode: 'optimize',
  }),
  actions: {
    setMode(mode: Mode) {
      this.mode = mode;
    },
  },
});
