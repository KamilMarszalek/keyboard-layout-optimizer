import { formatError } from '@/lib/error';
import { getCharFrequencies, getQwertyLayout } from '@/wasm/queries';
import { defineStore } from 'pinia';

import { fromCharFrequencyDto, fromLayoutDto } from './mapper';
import type { CharFrequency, Layout } from './types';

interface KeyboardState {
  standardQwertyLayout: Layout;
  editableLayout: Layout;
  isLoadingQwerty: boolean;
  showHeatmap: boolean;
  charFrequencies: CharFrequency[];
  layoutError: string | null;
}

export const useKeyboardStore = defineStore('keyboard', {
  state: (): KeyboardState => ({
    standardQwertyLayout: { mappings: [] },
    editableLayout: { mappings: [] },
    isLoadingQwerty: false,
    showHeatmap: false,
    charFrequencies: [],
    layoutError: null,
  }),
  actions: {
    reorderKey(from: number, to: number) {
      const mappings = this.editableLayout.mappings;
      if (from < 0 || to < 0 || from >= mappings.length || to >= mappings.length) {
        return;
      }
      [mappings[from], mappings[to]] = [mappings[to], mappings[from]];
    },

    resetEditableLayout() {
      this.editableLayout = {
        mappings: this.standardQwertyLayout.mappings.map((mapping) => ({ ...mapping })),
      };
    },

    async refreshCharFrequencies(text: string) {
      this.charFrequencies = text.trim()
        ? (await getCharFrequencies(text)).map(fromCharFrequencyDto)
        : [];
    },

    async loadStandardQwertyLayout() {
      if (this.isLoadingQwerty || this.standardQwertyLayout.mappings.length > 0) {
        return;
      }

      this.isLoadingQwerty = true;

      try {
        this.standardQwertyLayout = fromLayoutDto(await getQwertyLayout());
        this.resetEditableLayout();
      } catch (caught) {
        this.layoutError = `Failed to load the standard keyboard layout from WASM. ${formatError(caught)}`;
      } finally {
        this.isLoadingQwerty = false;
      }
    },
  },
});
