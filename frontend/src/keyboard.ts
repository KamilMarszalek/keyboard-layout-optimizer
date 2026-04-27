export function normalizeText(input: string): string {
  return input
    .split("")
    .filter((char) => /[a-zA-Z]/.test(char))
    .map((char) => char.toLowerCase())
    .join("");
}

export function standardKeyboardRows(): string[][] {
  return [
    ["`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "="],
    ["q", "w", "e", "r", "t", "y", "u", "i", "o", "p", "[", "]", "\\"],
    ["a", "s", "d", "f", "g", "h", "j", "k", "l", ";", "'"],
    ["z", "x", "c", "v", "b", "n", "m", ",", ".", "/"],
  ];
}

export function flattenKeyboardRows(rows: string[][]): string[] {
  return rows.flat();
}