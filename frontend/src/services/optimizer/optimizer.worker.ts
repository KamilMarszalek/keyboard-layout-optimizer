import { formatError } from '@/lib/error';

import type {
  OptimizerWorkerRequest,
  OptimizerWorkerResponse,
} from '../../wasm/optimizerWorkerTypesoptimizerWorkerTypes';
import { optimizeLayout } from '../services/optimizer/wasmClientasmClient';

function validateThreadSupport() {
  if (!globalThis.crossOriginIsolated) {
    throw new Error(
      'WASM threads require cross-origin isolation. Serve the app with COOP/COEP headers enabled.',
    );
  }

  if (typeof SharedArrayBuffer === 'undefined') {
    throw new Error('WASM threads require SharedArrayBuffer support in this browser context.');
  }
}

function postResponse(response: OptimizerWorkerResponse) {
  self.postMessage(response);
}

self.addEventListener('message', async ({ data }: MessageEvent<OptimizerWorkerRequest>) => {
  if (data.type !== 'optimize') {
    return;
  }

  try {
    validateThreadSupport();
    const result = await optimizeLayout(data.request);
    postResponse({ type: 'optimized', id: data.id, result });
  } catch (caught) {
    postResponse({ type: 'error', id: data.id, error: formatError(caught) });
  }
});
