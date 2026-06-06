import { formatError } from '@/lib/error';

import { optimizeLayout } from './queries';
import type { WorkerRequest, WorkerResponse } from './types';

function postResponse(response: WorkerResponse): void {
  self.postMessage(response);
}

self.addEventListener('message', async ({ data }: MessageEvent<WorkerRequest>) => {
  try {
    console.log('[worker] optimize start');
    const result = await optimizeLayout(data.request);
    console.log('[worker] optimize returned', result); // (A) WASM → JS
    postResponse({ type: 'SUCCESS', id: data.id, result });
    console.log('[worker] posted success'); // (B) worker → main (structured clone)
  } catch (caught) {
    console.error('[worker] caught', caught);
    postResponse({ type: 'ERROR', id: data.id, error: formatError(caught) });
  }
});
