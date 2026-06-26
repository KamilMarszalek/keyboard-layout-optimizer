import { describe, expect, it } from 'vitest';

import { corpusSchema, defaultCorpus } from './schema';

describe('corpusSchema', () => {
  it('accepts the default corpus', () => {
    expect(corpusSchema.safeParse(defaultCorpus).success).toBe(true);
  });

  it('trims surrounding whitespace', () => {
    const result = corpusSchema.safeParse({ text: '  hello  ' });
    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.data.text).toBe('hello');
    }
  });

  it('rejects an empty string', () => {
    expect(corpusSchema.safeParse({ text: '' }).success).toBe(false);
  });

  it('rejects whitespace-only text after trimming', () => {
    expect(corpusSchema.safeParse({ text: '   ' }).success).toBe(false);
  });

  it('rejects text longer than 2000 characters', () => {
    expect(corpusSchema.safeParse({ text: 'a'.repeat(2001) }).success).toBe(false);
  });

  it('accepts text exactly at the 2000 character limit', () => {
    expect(corpusSchema.safeParse({ text: 'a'.repeat(2000) }).success).toBe(true);
  });
});
