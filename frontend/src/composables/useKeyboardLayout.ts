import { computed, ref, type Ref } from "vue";
import { EXPECTED_LAYOUT_LENGTH } from "../keyboard";
import { getQwertyLayout } from "../wasmClient";
import type { OptimizeResultDto } from "../wasmTypes";

function formatError(caught: unknown): string {
  if (caught instanceof Error) {
    return caught.message;
  }

  return String(caught);
}

export function useKeyboardLayout(result: Ref<OptimizeResultDto | null>) {
  const standardQwertyLayout = ref<string[]>([]);
  const qwertyLayoutError = ref<string | null>(null);

  const layoutValidationMessage = computed(() => {
    if (qwertyLayoutError.value) {
      return qwertyLayoutError.value;
    }

    if (!result.value || result.value.bestLayout.length === EXPECTED_LAYOUT_LENGTH) {
      return null;
    }

    return `The optimizer returned ${result.value.bestLayout.length} keys; showing the standard QWERTY fallback.`;
  });

  const currentLayout = computed(() => {
    if (result.value?.bestLayout.length === EXPECTED_LAYOUT_LENGTH) {
      return result.value.bestLayout;
    }

    return standardQwertyLayout.value;
  });

  const layoutTitle = computed(() => (result.value ? "Optimized layout" : "Standard QWERTY layout"));

  async function loadStandardQwertyLayout() {
    qwertyLayoutError.value = null;

    try {
      const layout = await getQwertyLayout();

      if (layout.length !== EXPECTED_LAYOUT_LENGTH) {
        qwertyLayoutError.value = `QWERTY layout loaded ${layout.length} keys; expected ${EXPECTED_LAYOUT_LENGTH}.`;
        return;
      }

      standardQwertyLayout.value = layout;
    } catch (caught) {
      qwertyLayoutError.value = `Failed to load QWERTY layout: ${formatError(caught)}`;
    }
  }

  return {
    standardQwertyLayout,
    currentLayout,
    layoutTitle,
    layoutValidationMessage,
    expectedLayoutLength: EXPECTED_LAYOUT_LENGTH,
    loadStandardQwertyLayout,
  };
}
