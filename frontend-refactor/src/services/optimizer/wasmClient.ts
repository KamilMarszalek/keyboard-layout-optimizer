import init, { initThreadPool, optimize_layout, qwerty_layout } from '@/wasm/optimizer';
import type { OptimizeRequestDto, OptimizeResultDto } from './optimizer.dto';

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

export function initOptimizerWasm(): Promise<void> {
  if (!threadPoolPromise) {
    threadPoolPromise = (async () => {
      await initOptimizerModule();
      const availableThreads = navigator.hardwareConcurrency ?? 2;
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

  try {
    return optimize_layout(request) as OptimizeResultDto;
  } catch (caught) {
    throw errorWithCause(`WASM optimization failed: ${String(caught)}`, caught);
  }
}

// Must call initOptimizerModule (bare WASM init), NOT initOptimizerWasm —
// initOptimizerWasm also spawns the Rayon thread pool, which is unnecessary
// and potentially harmful when only the layout symbols are needed.
export async function getQwertyLayout(): Promise<string[]> {
  await initOptimizerModule();
  return qwerty_layout();
}
