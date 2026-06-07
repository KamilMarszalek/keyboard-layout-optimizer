import type { ControlField } from '@/lib/field';

import type { Corpus } from './schema';

export const corpusTextControl: ControlField<Corpus> = {
  key: 'text',
  label: 'Input text',
  description: 'Paste representative text to score layouts against.',
};
