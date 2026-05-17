import type { OptimizerWorkerRequest, OptimizerWorkerResponse } from "../optimizerWorkerTypes";
import type { OptimizeRequestDto, OptimizeResultDto } from "../wasmTypes";

type ActiveOptimization = {
  id: number;
  resolve: (result: OptimizeResultDto) => void;
  reject: (error: Error) => void;
};

function formatError(caught: unknown): string {
  if (caught instanceof Error) {
    return caught.message;
  }

  return String(caught);
}

function isolationError(): string | null {
  if (!window.crossOriginIsolated) {
    return "WASM threads require cross-origin isolation. Use the Vite dev or preview server with COOP/COEP headers enabled.";
  }

  if (typeof SharedArrayBuffer === "undefined") {
    return "WASM threads require SharedArrayBuffer support in this browser context.";
  }

  return null;
}

export function useOptimizerWorker() {
  let optimizerWorker: Worker | undefined;
  let activeOptimization: ActiveOptimization | undefined;
  let nextOptimizationId = 0;

  function handleWorkerMessage({ data }: MessageEvent<OptimizerWorkerResponse>) {
    if (!activeOptimization || data.id !== activeOptimization.id) {
      return;
    }

    const { resolve, reject } = activeOptimization;
    activeOptimization = undefined;

    if (data.type === "optimized") {
      resolve(data.result);
      return;
    }

    reject(new Error(data.error));
  }

  function handleWorkerFailure(event: ErrorEvent | MessageEvent) {
    if (activeOptimization) {
      activeOptimization.reject(
        new Error("Optimizer worker failed. Check the browser console for WASM worker details."),
      );
      activeOptimization = undefined;
    }

    optimizerWorker?.terminate();
    optimizerWorker = undefined;

    if ("preventDefault" in event) {
      event.preventDefault();
    }
  }

  function getOptimizerWorker(): Worker {
    if (optimizerWorker) {
      return optimizerWorker;
    }

    optimizerWorker = new Worker(new URL("../optimizer.worker.ts", import.meta.url), {
      type: "module",
    });
    optimizerWorker.addEventListener("message", handleWorkerMessage);
    optimizerWorker.addEventListener("error", handleWorkerFailure);
    optimizerWorker.addEventListener("messageerror", handleWorkerFailure);

    return optimizerWorker;
  }

  function optimizeInWorker(request: OptimizeRequestDto): Promise<OptimizeResultDto> {
    if (activeOptimization) {
      return Promise.reject(new Error("Optimization is already running."));
    }

    const threadError = isolationError();
    if (threadError) {
      return Promise.reject(new Error(threadError));
    }

    const id = ++nextOptimizationId;
    const message: OptimizerWorkerRequest = { type: "optimize", id, request };
    const worker = getOptimizerWorker();

    return new Promise((resolve, reject) => {
      activeOptimization = { id, resolve, reject };

      try {
        worker.postMessage(message);
      } catch (caught) {
        activeOptimization = undefined;
        reject(new Error(`Failed to send optimization request to worker: ${formatError(caught)}`));
      }
    });
  }

  function disposeWorker() {
    activeOptimization?.reject(new Error("Optimization cancelled because the view was closed."));
    optimizerWorker?.terminate();
    optimizerWorker = undefined;
    activeOptimization = undefined;
  }

  return {
    optimizeInWorker,
    disposeWorker,
  };
}
