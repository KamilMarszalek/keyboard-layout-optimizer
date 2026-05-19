import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { formatError } from "../lib/format";
import { EXPECTED_LAYOUT_LENGTH } from "../lib/keyboardLayout";
import { getQwertyLayout } from "../wasmClient";
import { useOptimizerStore } from "./optimizerStore";

export const useLayoutStore = defineStore("layout", () => {
  const standardQwertyLayout = ref<string[]>([]);
  const isLayoutLoading = ref(false);
  const layoutError = ref<string | null>(null);

  const optimizerStore = useOptimizerStore();
  const expectedLayoutLength = computed(() => EXPECTED_LAYOUT_LENGTH);

  const layoutValidationMessage = computed(() => {
    if (layoutError.value) {
      return layoutError.value;
    }

    if (!optimizerStore.result || optimizerStore.result.bestLayout.length === EXPECTED_LAYOUT_LENGTH) {
      return null;
    }

    return `The optimizer returned ${optimizerStore.result.bestLayout.length} keys; showing the standard QWERTY fallback.`;
  });

  const currentLayout = computed(() => {
    if (optimizerStore.result?.bestLayout.length === EXPECTED_LAYOUT_LENGTH) {
      return optimizerStore.result.bestLayout;
    }

    return standardQwertyLayout.value;
  });

  const layoutTitle = computed(() =>
    optimizerStore.result ? "Optimized layout" : "Standard QWERTY layout",
  );

  async function loadStandardQwertyLayout() {
    layoutError.value = null;
    isLayoutLoading.value = true;

    try {
      const layout = await getQwertyLayout();

      if (layout.length !== EXPECTED_LAYOUT_LENGTH) {
        layoutError.value = `QWERTY layout loaded ${layout.length} keys; expected ${EXPECTED_LAYOUT_LENGTH}.`;
        return;
      }

      standardQwertyLayout.value = layout;
    } catch (caught) {
      layoutError.value = `Failed to load QWERTY layout: ${formatError(caught)}`;
    } finally {
      isLayoutLoading.value = false;
    }
  }

  return {
    currentLayout,
    expectedLayoutLength,
    isLayoutLoading,
    layoutError,
    layoutTitle,
    layoutValidationMessage,
    loadStandardQwertyLayout,
    standardQwertyLayout,
  };
});
