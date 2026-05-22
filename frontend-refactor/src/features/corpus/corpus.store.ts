import { defineStore } from 'pinia';
import { defaultText } from './corpus.schema';

export const useCorpusStore = defineStore('corpus', {
  state: () => {
    return {
      text: defaultText,
    };
  },
  getters: {
    characterCount: (state) => state.text.trim().length,
  },
});
