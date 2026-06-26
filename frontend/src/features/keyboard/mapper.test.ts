import type { CharFrequencyDto, KeyMappingDto, LayoutDto } from '@/wasm/dto';
import { describe, expect, it } from 'vitest';

import { EXPECTED_LAYOUT_LENGTH } from './layout';
import { fromCharFrequencyDto, fromKeyMappingDto, fromLayoutDto } from './mapper';

function makeLayoutDto(length: number): LayoutDto {
  return {
    mappings: Array.from({ length }, (_, index) => ({
      base: String(index),
      shifted: '',
    })),
  };
}

describe('fromKeyMappingDto', () => {
  it('copies base and shifted fields', () => {
    const dto: KeyMappingDto = { base: 'a', shifted: 'A' };
    expect(fromKeyMappingDto(dto)).toEqual({ base: 'a', shifted: 'A' });
  });
});

describe('fromCharFrequencyDto', () => {
  it('copies key and frequency fields', () => {
    const dto: CharFrequencyDto = { key: 'e', frequency: 42 };
    expect(fromCharFrequencyDto(dto)).toEqual({ key: 'e', frequency: 42 });
  });
});

describe('fromLayoutDto', () => {
  it('returns the layout when it has the expected length', () => {
    const dto = makeLayoutDto(EXPECTED_LAYOUT_LENGTH);
    expect(fromLayoutDto(dto).mappings).toHaveLength(EXPECTED_LAYOUT_LENGTH);
  });

  it('throws when the layout is too short', () => {
    const dto = makeLayoutDto(EXPECTED_LAYOUT_LENGTH - 1);
    expect(() => fromLayoutDto(dto)).toThrow(/standard ANSI format/);
    expect(() => fromLayoutDto(dto)).toThrow(String(EXPECTED_LAYOUT_LENGTH - 1));
  });

  it('throws when the layout is too long', () => {
    const dto = makeLayoutDto(EXPECTED_LAYOUT_LENGTH + 1);
    expect(() => fromLayoutDto(dto)).toThrow(/standard ANSI format/);
  });

  it('throws on an empty layout', () => {
    expect(() => fromLayoutDto({ mappings: [] })).toThrow();
  });
});
