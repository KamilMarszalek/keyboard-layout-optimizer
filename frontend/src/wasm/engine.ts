import { once } from '@/lib/once';
import init, { initThreadPool } from '@/pkg/optimizer';

export async function initWasmCore(): Promise<void> {
  return once(async () => {
    await init();
  })();
}

export async function initWasmThreadpool(): Promise<void> {
  return once(async () => {
    if (!globalThis.crossOriginIsolated) {
      throw new Error(
        'WASM threads require cross-origin isolation. Serve the app with COOP/COEP headers enabled.',
      );
    }
    const threads = Math.min(navigator.hardwareConcurrency, 2);
    await initThreadPool(threads);
  })();
}

export async function runWasm<T, A>(
  ensureInit: (arg?: A) => Promise<void>,
  fn: () => T,
): Promise<T> {
  await ensureInit();
  return fn();
}
