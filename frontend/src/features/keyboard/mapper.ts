import type { CharFrequencyDto, KeyMappingDto, LayoutDto } from '@/wasm/dto';

import { EXPECTED_LAYOUT_LENGTH, hasExpectedLayoutLength } from './layout';
import type { CharFrequency, KeyMapping, Layout } from './types';

export function fromKeyMappingDto(value: KeyMappingDto): KeyMapping {
  return {
    base: value.base,
    shifted: value.shifted,
  };
}

export function fromLayoutDto(value: LayoutDto): Layout {
  if (!hasExpectedLayoutLength(value)) {
    throw new Error(
      `Layout is not in standard ANSI format; expected ${EXPECTED_LAYOUT_LENGTH} got ${value.mappings.length}`,
    );
  }
  return {
    mappings: value.mappings,
  };
}

export function fromCharFrequencyDto(value: CharFrequencyDto): CharFrequency {
  return {
    key: value.key,
    frequency: value.frequency,
  };
}
