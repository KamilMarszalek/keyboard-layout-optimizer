import type { ControlField } from '@/lib/field';

import type { Corpus } from './corpus.schema';

export const corpusTextControl: ControlField<Corpus> = {
  key: 'text',
  label: 'Input text',
  description: 'Paste representative text for the optimizer to evaluate.',
};
