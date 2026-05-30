import path from 'node:path';
import { defineConfig, type UserConfig } from 'vite';
import type { InlineConfig } from 'vitest/node';

const config = {
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  test: {
    globals: true,
  },
} satisfies UserConfig & { test: InlineConfig };

export default defineConfig(config);
