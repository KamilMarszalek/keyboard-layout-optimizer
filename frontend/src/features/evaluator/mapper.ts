import type { MetricsWeights } from '@/features/config/schema';
import type { EvaluateRequestDto } from '@/wasm/dto';

export function toEvaluateRequestDto(
  keys: string[],
  text: string,
  weights: MetricsWeights,
): EvaluateRequestDto {
  return {
    keys,
    text,
    weights,
  };
}
