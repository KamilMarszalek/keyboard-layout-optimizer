import { defineStore } from 'pinia';

import { defaultText } from './schema';

interface CorpusState {
  text: string;
}

export const useCorpusStore = defineStore('corpus', {
  state: (): CorpusState => ({
    text: defaultText,
  }),
  actions: {
    setText(text: string) {
      this.text = text;
    },
  },
});
