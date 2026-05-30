import { describe, expect, it } from 'vitest';
import { toOptimizeRequestDto } from './optimizer.mapper';
import type { OptimizeRequest } from './optimizer.schema';

type OptimizeRequestOverrides = {
  corpus?: Partial<OptimizeRequest['corpus']>;
  config?: Partial<Omit<OptimizeRequest['config'], 'weights' | 'annealing'>> & {
    weights?: Partial<OptimizeRequest['config']['weights']>;
    annealing?: Partial<OptimizeRequest['config']['annealing']>;
  };
};

function makeRequest(overrides: OptimizeRequestOverrides = {}): OptimizeRequest {
  const base: OptimizeRequest = {
    corpus: {
      text: 'sample corpus',
    },
    config: {
      weights: {
        sameFingerBigrams: 1,
        fingerDistance: 2,
        homeRowUsage: 3,
        handAlternation: 4,
        rowJumping: 5,
      },
      annealing: {
        tStart: 1,
        tMin: 0.001,
        alpha: 0.995,
        iterationsPerTemp: 99.6,
      },
      seed: 42,
    },
  };

  return {
    ...base,
    ...overrides,
    corpus: {
      ...base.corpus,
      ...overrides.corpus,
    },
    config: {
      ...base.config,
      ...overrides.config,
      weights: {
        ...base.config.weights,
        ...overrides.config?.weights,
      },
      annealing: {
        ...base.config.annealing,
        ...overrides.config?.annealing,
      },
    },
  };
}

describe('toOptimizeRequestDto', () => {
  it('passes corpus text through unchanged (trimming is the schema layer responsibility)', () => {
    expect(toOptimizeRequestDto(makeRequest()).text).toBe('sample corpus');
  });

  it('maps null seed to undefined', () => {
    expect(toOptimizeRequestDto(makeRequest({ config: { seed: null } })).seed).toBeUndefined();
  });

  it('keeps numeric seed values', () => {
    expect(toOptimizeRequestDto(makeRequest({ config: { seed: 7 } })).seed).toBe(7);
  });

  it('rounds iterations per temperature', () => {
    expect(
      toOptimizeRequestDto(
        makeRequest({
          config: {
            annealing: {
              iterationsPerTemp: 12.6,
            },
          },
        }),
      ).annealing.iterationsPerTemp,
    ).toBe(13);
  });

  it('clamps iterations per temperature to at least one', () => {
    expect(
      toOptimizeRequestDto(
        makeRequest({
          config: {
            annealing: {
              iterationsPerTemp: 0,
            },
          },
        }),
      ).annealing.iterationsPerTemp,
    ).toBe(1);
  });

  it('maps weights and annealing values to the WASM DTO shape', () => {
    const dto = toOptimizeRequestDto(makeRequest());

    expect(dto.weights).toEqual({
      sameFingerBigrams: 1,
      fingerDistance: 2,
      homeRowUsage: 3,
      handAlternation: 4,
      rowJumping: 5,
    });
    expect(dto.annealing).toEqual({
      tStart: 1,
      tMin: 0.001,
      alpha: 0.995,
      iterationsPerTemp: 100,
    });
  });
});
