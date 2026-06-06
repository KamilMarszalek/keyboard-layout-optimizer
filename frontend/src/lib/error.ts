export function formatError(caught: unknown): string {
  return caught instanceof Error ? caught.message : String(caught);
}
