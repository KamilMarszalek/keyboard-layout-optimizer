import type { OptimizeRequestDto, OptimizeResultDto } from "./wasmTypes";

export type OptimizerWorkerRequest = {
  type: "optimize";
  id: number;
  request: OptimizeRequestDto;
};

export type OptimizerWorkerResponse =
  | {
      type: "optimized";
      id: number;
      result: OptimizeResultDto;
    }
  | {
      type: "error";
      id: number;
      error: string;
    };
