import { describe, expect, it } from "vitest";
import { flattenKeyboardRows, standardKeyboardRows } from "./keyboard";

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