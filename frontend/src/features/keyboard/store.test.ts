import type { CharFrequencyDto, LayoutDto } from '@/wasm/dto';
import { createPinia, setActivePinia } from 'pinia';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import { EXPECTED_LAYOUT_LENGTH } from './layout';
import { useKeyboardStore } from './store';

const getQwertyLayout = vi.fn<() => Promise<LayoutDto>>();
const getCharFrequencies = vi.fn<(text: string) => Promise<CharFrequencyDto[]>>();

vi.mock('@/wasm/queries', () => ({
  getQwertyLayout: () => getQwertyLayout(),
  getCharFrequencies: (text: string) => getCharFrequencies(text),
}));

function makeLayoutDto(length = EXPECTED_LAYOUT_LENGTH): LayoutDto {
  return {
    mappings: Array.from({ length }, (_, index) => ({
      base: String(index),
      shifted: '',
    })),
  };
}

beforeEach(() => {
  setActivePinia(createPinia());
  getQwertyLayout.mockReset();
  getCharFrequencies.mockReset();
});

describe('reorderKey', () => {
  it('swaps two mappings in the editable layout', () => {
    const store = useKeyboardStore();
    store.editableLayout = {
      mappings: [
        { base: 'a', shifted: '' },
        { base: 'b', shifted: '' },
        { base: 'c', shifted: '' },
      ],
    };

    store.reorderKey(0, 2);

    expect(store.editableLayout.mappings.map((m) => m.base)).toEqual(['c', 'b', 'a']);
  });
});

describe('resetEditableLayout', () => {
  it('deep-copies the standard layout so edits do not leak back', () => {
    const store = useKeyboardStore();
    store.standardQwertyLayout = {
      mappings: [
        { base: 'a', shifted: '' },
        { base: 'b', shifted: '' },
      ],
    };

    store.resetEditableLayout();
    store.editableLayout.mappings[0].base = 'z';

    expect(store.standardQwertyLayout.mappings[0].base).toBe('a');
  });
});

describe('refreshCharFrequencies', () => {
  it('loads and maps frequencies for non-empty text', async () => {
    getCharFrequencies.mockResolvedValue([{ key: 'a', frequency: 3 }]);
    const store = useKeyboardStore();

    await store.refreshCharFrequencies('abc');

    expect(getCharFrequencies).toHaveBeenCalledWith('abc');
    expect(store.charFrequencies).toEqual([{ key: 'a', frequency: 3 }]);
  });

  it('clears frequencies for blank text without calling WASM', async () => {
    const store = useKeyboardStore();
    store.charFrequencies = [{ key: 'a', frequency: 3 }];

    await store.refreshCharFrequencies('   ');

    expect(getCharFrequencies).not.toHaveBeenCalled();
    expect(store.charFrequencies).toEqual([]);
  });
});

describe('loadStandardQwertyLayout', () => {
  it('loads the qwerty layout and resets the editable copy', async () => {
    getQwertyLayout.mockResolvedValue(makeLayoutDto());
    const store = useKeyboardStore();

    await store.loadStandardQwertyLayout();

    expect(getQwertyLayout).toHaveBeenCalledTimes(1);
    expect(store.standardQwertyLayout.mappings).toHaveLength(EXPECTED_LAYOUT_LENGTH);
    expect(store.editableLayout.mappings).toHaveLength(EXPECTED_LAYOUT_LENGTH);
    expect(store.layoutError).toBeNull();
    expect(store.isLoadingQwerty).toBe(false);
  });

  it('does not reload when a layout is already present', async () => {
    const store = useKeyboardStore();
    store.standardQwertyLayout = makeLayoutDto();

    await store.loadStandardQwertyLayout();

    expect(getQwertyLayout).not.toHaveBeenCalled();
  });

  it('records an error when the layout fails validation', async () => {
    getQwertyLayout.mockResolvedValue(makeLayoutDto(EXPECTED_LAYOUT_LENGTH - 1));
    const store = useKeyboardStore();

    await store.loadStandardQwertyLayout();

    expect(store.layoutError).toMatch(/Failed to load the standard keyboard layout/);
    expect(store.isLoadingQwerty).toBe(false);
  });
});
