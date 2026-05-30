import { defineStore } from 'pinia';
import { formatError } from '@/lib/format';
import { getQwertyLayout } from '@/services/optimizer/wasmClient';
import { defaultText } from '@/features/corpus/corpus.schema';
import { EXPECTED_LAYOUT_LENGTH } from './keyboardLayout';

interface KeyboardState {
  standardQwertyLayout: string[];
  isLoadingQwerty: boolean;
  layoutError: string | null;
  corpusText: string;
}

export const useKeyboardStore = defineStore('keyboard', {
  state: (): KeyboardState => ({
    standardQwertyLayout: [],
    isLoadingQwerty: false,
    layoutError: null,
    corpusText: defaultText,
  }),
  getters: {
    expectedLayoutLength: () => EXPECTED_LAYOUT_LENGTH,
  },
  actions: {
    setCorpusText(text: string) {
      this.corpusText = text;
    },
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
