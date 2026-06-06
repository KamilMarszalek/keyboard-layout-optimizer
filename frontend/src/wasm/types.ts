import type { OptimizeRequestDto, OptimizeResultDto } from './dto';

export interface WorkerRequest {
  type: 'optimize';
  id: number;
  request: OptimizeRequestDto;
}

export interface WorkerSuccessResponse {
  type: 'SUCCESS';
  id: number;
  result: OptimizeResultDto;
}

export interface WorkerErrorResponse {
  type: 'ERROR';
  id: number;
  error: string;
}

export type WorkerResponse = WorkerSuccessResponse | WorkerErrorResponse;
