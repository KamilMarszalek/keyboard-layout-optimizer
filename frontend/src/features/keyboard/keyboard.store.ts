import { defineStore } from 'pinia';
import { formatError } from '@/lib/format';
import { getQwertyLayout } from '@/services/optimizer/wasmClient';
import { EXPECTED_LAYOUT_LENGTH } from './keyboardLayout';

interface KeyboardState {
  standardQwertyLayout: string[];
  isLoadingQwerty: boolean;
  layoutError: string | null;
}

export const useKeyboardStore = defineStore('keyboard', {
  state: (): KeyboardState => ({
    standardQwertyLayout: [],
    isLoadingQwerty: false,
    layoutError: null,
  }),
  getters: {
    expectedLayoutLength: () => EXPECTED_LAYOUT_LENGTH,
  },
  actions: {
    async loadStandardQwertyLayout() {
      if (this.isLoadingQwerty || this.standardQwertyLayout.length > 0) {
        return;
      }

      this.isLoadingQwerty = true;
      this.layoutError = null;

      try {
        this.standardQwertyLayout = await getQwertyLayout();
      } catch (caught) {
        this.layoutError = `Failed to load the standard keyboard layout from WASM. ${formatError(caught)}`;
      } finally {
        this.isLoadingQwerty = false;
      }
    },
  },
});
