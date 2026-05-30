import type { OptimizeRequestDto, OptimizeResultDto } from './optimizer.dto';

export type OptimizerWorkerRequest = {
  type: 'optimize';
  id: number;
  request: OptimizeRequestDto;
};

export type OptimizerWorkerResponse =
  | {
      type: 'optimized';
      id: number;
      result: OptimizeResultDto;
    }
  | {
      type: 'error';
      id: number;
      error: string;
    };
