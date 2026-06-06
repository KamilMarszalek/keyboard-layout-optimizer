export function themeColor(token: string): string {
  return getComputedStyle(document.documentElement).getPropertyValue(token).trim();
}
