import { formatError } from '@/lib/format';
import type { OptimizeRequestDto, OptimizeResultDto } from './optimizer.dto';
import type { OptimizerWorkerRequest, OptimizerWorkerResponse } from './optimizerWorkerTypes';

type ActiveOptimization = {
  id: number;
  resolve: (result: OptimizeResultDto) => void;
  reject: (error: Error) => void;
};

class OptimizerWorkerClient {
  private worker: Worker | undefined;
  private activeOptimization: ActiveOptimization | undefined;
  private nextOptimizationId = 0;

  optimize(request: OptimizeRequestDto): Promise<OptimizeResultDto> {
    if (this.activeOptimization) {
      return Promise.reject(new Error('Optimization is already running.'));
    }

    const id = ++this.nextOptimizationId;
    const message: OptimizerWorkerRequest = { type: 'optimize', id, request };
    const worker = this.getWorker();

    return new Promise((resolve, reject) => {
      this.activeOptimization = { id, resolve, reject };

      try {
        worker.postMessage(message);
      } catch (caught) {
        this.activeOptimization = undefined;
        reject(new Error(`Failed to send optimization request to worker: ${formatError(caught)}`));
      }
    });
  }

  dispose() {
    const active = this.activeOptimization;
    this.activeOptimization = undefined;
    active?.reject(new Error('Optimization cancelled because the view was closed.'));
    this.worker?.terminate();
    this.worker = undefined;
  }

  private getWorker(): Worker {
    if (this.worker) {
      return this.worker;
    }

    this.worker = new Worker(new URL('./optimizer.worker.ts', import.meta.url), {
      type: 'module',
    });
    this.worker.addEventListener('message', this.handleWorkerMessage);
    this.worker.addEventListener('error', this.handleWorkerFailure);
    this.worker.addEventListener('messageerror', this.handleWorkerFailure);

    return this.worker;
  }

  private handleWorkerMessage = ({ data }: MessageEvent<OptimizerWorkerResponse>) => {
    if (!this.activeOptimization || data.id !== this.activeOptimization.id) {
      return;
    }

    const { resolve, reject } = this.activeOptimization;
    this.activeOptimization = undefined;

    if (data.type === 'optimized') {
      resolve(data.result);
      return;
    }

    reject(new Error(data.error));
  };

  private handleWorkerFailure = () => {
    if (this.activeOptimization) {
      this.activeOptimization.reject(
        new Error('Optimizer worker failed. Check the browser console for WASM worker details.'),
      );
      this.activeOptimization = undefined;
    }

    this.worker?.terminate();
    this.worker = undefined;

  };
}

const optimizerWorkerClient = new OptimizerWorkerClient();

export function optimizeInWorker(request: OptimizeRequestDto): Promise<OptimizeResultDto> {
  return optimizerWorkerClient.optimize(request);
}

export function disposeOptimizerWorker() {
  optimizerWorkerClient.dispose();
}
