import { z } from 'zod';

export const defaultText =
  'The quick brown fox jumps over the lazy dog while the keyboard optimizer searches for a smoother typing layout.';

export const corpusSchema = z.object({
  text: z.string().trim().min(1).max(2000),
});
export type Corpus = z.infer<typeof corpusSchema>;
export const defaultCorpus: Corpus = {
  text: defaultText,
};
