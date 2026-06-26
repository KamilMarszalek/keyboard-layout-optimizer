import { describe, expect, it } from 'vitest';

import { METRIC_WEIGHT_MAX, METRIC_WEIGHT_MIN } from './constants';
import { annealingSchema, defaultAnnealingParams, defaultWeights, weightsSchema } from './schema';

describe('weightsSchema', () => {
  it('accepts the default weights', () => {
    expect(weightsSchema.safeParse(defaultWeights).success).toBe(true);
  });

  it('accepts weights at the bounds', () => {
    const atMin = { ...defaultWeights, fingerDistance: METRIC_WEIGHT_MIN };
    const atMax = { ...defaultWeights, fingerDistance: METRIC_WEIGHT_MAX };
    expect(weightsSchema.safeParse(atMin).success).toBe(true);
    expect(weightsSchema.safeParse(atMax).success).toBe(true);
  });

  it('rejects weights below the minimum', () => {
    const tooLow = { ...defaultWeights, fingerDistance: METRIC_WEIGHT_MIN - 1 };
    expect(weightsSchema.safeParse(tooLow).success).toBe(false);
  });

  it('rejects weights above the maximum', () => {
    const tooHigh = { ...defaultWeights, fingerDistance: METRIC_WEIGHT_MAX + 1 };
    expect(weightsSchema.safeParse(tooHigh).success).toBe(false);
  });

  it('rejects a missing metric', () => {
    const { rowJumping, ...incomplete } = defaultWeights;
    void rowJumping;
    expect(weightsSchema.safeParse(incomplete).success).toBe(false);
  });
});

describe('annealingSchema', () => {
  it('accepts the default annealing params', () => {
    expect(annealingSchema.safeParse(defaultAnnealingParams).success).toBe(true);
  });

  it('rejects a non-positive start temperature', () => {
    expect(annealingSchema.safeParse({ ...defaultAnnealingParams, tStart: 0 }).success).toBe(false);
  });

  it('rejects a non-positive minimum temperature', () => {
    expect(annealingSchema.safeParse({ ...defaultAnnealingParams, tMin: 0 }).success).toBe(false);
  });

  it('rejects alpha equal to or above 1', () => {
    expect(annealingSchema.safeParse({ ...defaultAnnealingParams, alpha: 1 }).success).toBe(false);
  });

  it('accepts alpha of 0', () => {
    expect(annealingSchema.safeParse({ ...defaultAnnealingParams, alpha: 0 }).success).toBe(true);
  });

  it('rejects a non-integer iterationsPerTemp', () => {
    const result = annealingSchema.safeParse({
      ...defaultAnnealingParams,
      iterationsPerTemp: 1.5,
    });
    expect(result.success).toBe(false);
  });

  it('rejects iterationsPerTemp below 1', () => {
    const result = annealingSchema.safeParse({ ...defaultAnnealingParams, iterationsPerTemp: 0 });
    expect(result.success).toBe(false);
  });

  it('rejects tMin greater than or equal to tStart with a targeted message', () => {
    const result = annealingSchema.safeParse({
      ...defaultAnnealingParams,
      tStart: 0.5,
      tMin: 0.5,
    });
    expect(result.success).toBe(false);
    if (!result.success) {
      const issue = result.error.issues.find((i) => i.path.includes('tMin'));
      expect(issue?.message).toMatch(/lower than start temperature/);
    }
  });
});
