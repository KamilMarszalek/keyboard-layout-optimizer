import { describe, expect, it } from "vitest";
import { normalizeText } from "./keyboard";

describe("normalizeText", () => {
  it("keeps only letters and lowercases them", () => {
    expect(normalizeText("Ala ma Kota! 123")).toBe("alamakota");
  });
});

