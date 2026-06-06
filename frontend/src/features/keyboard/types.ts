export interface KeyMapping {
  base: string;
  shifted: string;
}

export interface Layout {
  mappings: KeyMapping[];
}

export interface CharFrequency {
  key: string;
  frequency: number;
}
