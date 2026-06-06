import { once } from '@/lib/once';
import init, { initThreadPool } from '@/pkg/optimizer';

const ensureCore = once(async () => {
  await init();
});

const ensureThreadpool = once(async () => {
  if (!globalThis.crossOriginIsolated) {
    throw new Error(
      'WASM threads require cross-origin isolation. Serve the app with COOP/COEP headers enabled.',
    );
  }
  const threads = Math.min(navigator.hardwareConcurrency, 8);
  await initThreadPool(threads);
});

export function initWasmCore(): Promise<void> {
  return ensureCore();
}

export function initWasmThreadpool(): Promise<void> {
  return ensureThreadpool();
}

export async function runWasm<T, A>(
  ensureInit: (arg?: A) => Promise<void>,
  fn: () => T,
): Promise<T> {
  await ensureInit();
  return fn();
}
