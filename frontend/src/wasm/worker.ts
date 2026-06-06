import { formatError } from '@/lib/error';

import { optimizeLayout } from './queries';
import type { WorkerRequest, WorkerResponse } from './types';

function postResponse(response: WorkerResponse): void {
  self.postMessage(response);
}

self.addEventListener('message', async ({ data }: MessageEvent<WorkerRequest>) => {
  try {
    const result = await optimizeLayout(data.request);
    postResponse({ type: 'SUCCESS', id: data.id, result });
  } catch (caught) {
    postResponse({ type: 'ERROR', id: data.id, error: formatError(caught) });
  }
});
