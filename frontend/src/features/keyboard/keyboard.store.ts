import { defineStore } from 'pinia';
import { formatError } from '@/lib/format';
import { getQwertyLayout } from '@/services/optimizer/wasmClient';
import { useResultsStore } from '@/features/results/results.store';
import { EXPECTED_LAYOUT_LENGTH, hasExpectedLayoutLength } from './keyboardLayout';

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
    optimizedLayout(): string[] | undefined {
      return useResultsStore().result?.bestLayout;
    },
    layoutValidationMessage(): string | null {
      if (this.layoutError) {
        return this.layoutError;
      }

      const optimizedLayout = this.optimizedLayout;
      if (optimizedLayout && !hasExpectedLayoutLength(optimizedLayout)) {
        return `The optimizer returned ${optimizedLayout.length} keys; expected ${EXPECTED_LAYOUT_LENGTH}.`;
      }

      if (
        !optimizedLayout &&
        this.standardQwertyLayout.length > 0 &&
        !hasExpectedLayoutLength(this.standardQwertyLayout)
      ) {
        return `The standard keyboard layout loaded ${this.standardQwertyLayout.length} keys; expected ${EXPECTED_LAYOUT_LENGTH}.`;
      }

      return null;
    },
    currentLayout(): string[] {
      const optimizedLayout = this.optimizedLayout;

      if (optimizedLayout) {
        return hasExpectedLayoutLength(optimizedLayout) ? optimizedLayout : [];
      }

      return hasExpectedLayoutLength(this.standardQwertyLayout) ? this.standardQwertyLayout : [];
    },
    layoutTitle(): string {
      return this.optimizedLayout ? 'Optimized layout' : 'Standard QWERTY layout';
    },
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
