import type { CharFrequencyDto, KeyMappingDto, LayoutDto } from '@/wasm/dto';

import type { CharFrequency, KeyMapping, Layout } from './types';

export function fromKeyMappingDto(value: KeyMappingDto): KeyMapping {
  return {
    base: value.base,
    shifted: value.shifted,
  };
}

export function fromLayoutDto(value: LayoutDto): Layout {
  return {
    mappings: value.mappings.map(fromKeyMappingDto),
  };
}

export function fromCharFrequencyDto(value: CharFrequencyDto): CharFrequency {
  return {
    key: value.key,
    frequency: value.frequency,
  };
}
