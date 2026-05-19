export function formatNumber(value: number, maximumFractionDigits = 6): string {
  return value.toLocaleString(undefined, {
    maximumFractionDigits,
  });
}

export function formatError(caught: unknown): string {
  if (caught instanceof Error) {
    return caught.message;
  }

  return String(caught);
}
