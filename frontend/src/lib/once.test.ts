import { describe, expect, it, vi } from 'vitest';

import { once } from './once';

describe('once', () => {
  it('invokes the wrapped function only once for concurrent calls', async () => {
    const fn = vi.fn(async () => 'value');
    const wrapped = once(fn);

    const [a, b] = await Promise.all([wrapped(), wrapped()]);

    expect(a).toBe('value');
    expect(b).toBe('value');
    expect(fn).toHaveBeenCalledTimes(1);
  });

  it('caches the resolved promise across later calls', async () => {
    const fn = vi.fn(async () => 'value');
    const wrapped = once(fn);

    await wrapped();
    await wrapped();

    expect(fn).toHaveBeenCalledTimes(1);
  });

  it('forwards the first argument to the wrapped function', async () => {
    const fn = vi.fn(async (arg?: number) => arg);
    const wrapped = once(fn);

    await expect(wrapped(7)).resolves.toBe(7);
    expect(fn).toHaveBeenCalledWith(7);
  });

  it('clears the cache on rejection so the next call retries', async () => {
    const fn = vi
      .fn<() => Promise<string>>()
      .mockRejectedValueOnce(new Error('boom'))
      .mockResolvedValueOnce('recovered');
    const wrapped = once(fn);

    await expect(wrapped()).rejects.toThrow('boom');
    await expect(wrapped()).resolves.toBe('recovered');
    expect(fn).toHaveBeenCalledTimes(2);
  });
});
