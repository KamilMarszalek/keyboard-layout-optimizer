import { describe, expect, it } from "vitest";
import {
  EXPECTED_LAYOUT_LENGTH,
  flattenKeyboardRows,
  layoutToRows,
  standardKeyboardRows,
} from "./keyboard";

describe("standardKeyboardRows", () => {
  it("contains 47 optimized keys", () => {
    expect(flattenKeyboardRows(standardKeyboardRows())).toHaveLength(EXPECTED_LAYOUT_LENGTH);
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

describe("layoutToRows", () => {
  it("splits a flat layout into physical keyboard rows", () => {
    const layout = Array.from({ length: EXPECTED_LAYOUT_LENGTH }, (_, index) => String(index));

    expect(layoutToRows(layout).map((row) => row.length)).toEqual([13, 13, 11, 10]);
    expect(layoutToRows(layout)[0][0]).toBe("0");
    expect(layoutToRows(layout)[3][9]).toBe("46");
  });
});
