import init, {
  initThreadPool,
  optimize_layout,
  qwerty_layout,
} from "./wasm/optimizer";

import type { OptimizeRequestDto, OptimizeResultDto } from "./wasmTypes";

let initPromise: Promise<void> | undefined;
let threadPoolPromise: Promise<void> | undefined;

function errorWithCause(message: string, cause: unknown): Error {
  const error = new Error(message) as Error & { cause: unknown };
  error.cause = cause;
  return error;
}

function initOptimizerModule(): Promise<void> {
  if (!initPromise) {
    initPromise = init().then(() => undefined);
  }

  return initPromise;
}

export function initOptimizerWasm(): Promise<void> {
  if (!threadPoolPromise) {
    threadPoolPromise = (async () => {
      await initOptimizerModule();
      const availableThreads = navigator.hardwareConcurrency ?? 2;
      const threadCount = Math.min(2, availableThreads);

      await initThreadPool(threadCount);
    })();
  }

  return threadPoolPromise;
}

export async function optimizeLayout(
  request: OptimizeRequestDto,
): Promise<OptimizeResultDto> {
  await initOptimizerWasm();

  try {
    return optimize_layout(request) as OptimizeResultDto;
  } catch (error) {
    throw errorWithCause(`WASM optimization failed: ${String(error)}`, error);
  }
}

export async function getQwertyLayout(): Promise<string[]> {
  await initOptimizerModule();
  return qwerty_layout();
}
