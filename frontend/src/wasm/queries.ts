import { evaluate_layout, get_char_freq, optimize_layout, qwerty_layout } from '@/pkg/optimizer';

import type {
  CharFrequencyDto,
  EvaluateRequestDto,
  EvaluateResultDto,
  LayoutDto,
  OptimizeRequestDto,
  OptimizeResultDto,
} from './dto';
import { initWasmCore, initWasmThreadpool, runWasm } from './engine';

export async function optimizeLayout(request: OptimizeRequestDto): Promise<OptimizeResultDto> {
  const init = async () => {
    await initWasmCore();
    await initWasmThreadpool();
  };
  return runWasm(init, () => optimize_layout(request));
}

export async function evaluateLayout(request: EvaluateRequestDto): Promise<EvaluateResultDto> {
  const init = initWasmCore;
  return runWasm(init, () => evaluate_layout(request));
}

export async function getQwertyLayout(): Promise<LayoutDto> {
  const init = initWasmCore;
  return runWasm(init, qwerty_layout);
}

export async function getCharFrequencies(text: string): Promise<CharFrequencyDto[]> {
  const init = initWasmCore;
  return runWasm(init, () => get_char_freq(text));
}
