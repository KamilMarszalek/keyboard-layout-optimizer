import { formatError } from '@/lib/error';
import { getQwertyLayout } from '@/wasm/queries';
import { defineStore } from 'pinia';

import { fromLayoutDto } from './mapper';
import type { Layout } from './types';

interface KeyboardState {
  standardQwertyLayout: Layout;
  isLoadingQwerty: boolean;
  layoutError: string | null;
}

export const useKeyboardStore = defineStore('keyboard', {
  state: (): KeyboardState => ({
    standardQwertyLayout: { mappings: [] },
    isLoadingQwerty: false,
    layoutError: null,
  }),
  actions: {
    async loadStandardQwertyLayout() {
      if (this.isLoadingQwerty || this.standardQwertyLayout.mappings.length > 0) {
        return;
      }

      this.isLoadingQwerty = true;
      this.layoutError = null;

      try {
        this.standardQwertyLayout = fromLayoutDto(await getQwertyLayout());
      } catch (caught) {
        this.layoutError = `Failed to load the standard keyboard layout from WASM. ${formatError(caught)}`;
      } finally {
        this.isLoadingQwerty = false;
      }
    },
  },
});
