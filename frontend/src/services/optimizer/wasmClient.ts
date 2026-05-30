import init, { initThreadPool, optimize_layout, qwerty_layout } from '@/wasm/optimizer';
import { optimizeResultSchema, type OptimizeRequestDto, type OptimizeResultDto } from './optimizer.dto';

let initPromise: Promise<void> | undefined;
let threadPoolPromise: Promise<void> | undefined;

function errorWithCause(message: string, cause: unknown): Error {
  const error = new Error(message) as Error & { cause: unknown };
  error.cause = cause;
  return error;
}

function initOptimizerModule(): Promise<void> {
  if (!initPromise) {
    initPromise = init()
      .then(() => undefined)
      .catch((caught) => {
        initPromise = undefined;
        throw caught;
      });
  }

  return initPromise;
}

function initOptimizerWasm(): Promise<void> {
  if (!threadPoolPromise) {
    threadPoolPromise = (async () => {
      await initOptimizerModule();
      const availableThreads = navigator.hardwareConcurrency ?? 2;
      // Cap matches the length of initial_layouts in wasm.rs: two starting points
      // (QWERTY + Dvorak) run in parallel, so more than 2 threads add no throughput.
      // Should be changed if more starting layouts are added.
      const threadCount = Math.min(availableThreads, 2);

      await initThreadPool(threadCount);
    })().catch((caught) => {
      threadPoolPromise = undefined;
      throw caught;
    });
  }

  return threadPoolPromise;
}

export async function optimizeLayout(
  request: OptimizeRequestDto,
): Promise<OptimizeResultDto> {
  await initOptimizerWasm();

  let raw: unknown;
  try {
    raw = optimize_layout(request);
  } catch (caught) {
    throw errorWithCause(`WASM optimization failed: ${String(caught)}`, caught);
  }

  return optimizeResultSchema.parse(raw);
}

// Must call initOptimizerModule (bare WASM init), NOT initOptimizerWasm —
// initOptimizerWasm also spawns the Rayon thread pool, which is unnecessary
// and potentially harmful when only the layout symbols are needed.
export async function getQwertyLayout(): Promise<string[]> {
  await initOptimizerModule();
  return qwerty_layout();
}
