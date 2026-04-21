export function normalizeText(input: string): string {
  return input
    .split("")
    .filter((char) => /[a-zA-Z]/.test(char))
    .map((char) => char.toLowerCase())
    .join("");
}