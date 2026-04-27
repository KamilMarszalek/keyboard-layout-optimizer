import { describe, expect, it } from "vitest";
import { flattenKeyboardRows, normalizeText, standardKeyboardRows } from "./keyboard";

describe("normalizeText", () => {
  it("keeps only letters and lowercases them", () => {
    expect(normalizeText("Ala ma Kota! 123")).toBe("alamakota");
  });
});

describe("standardKeyboardRows", () => {
  it("contains 47 optimized keys", () => {
    expect(flattenKeyboardRows(standardKeyboardRows())).toHaveLength(47);
  });

  it("contains expected home row keys", () => {
    expect(standardKeyboardRows()[2]).toEqual([
      "a",
      "s",
      "d",
      "f",
      "g",
      "h",
      "j",
      "k",
      "l",
      ";",
      "'",
    ]);
  });
});