export type ControlField<T> = {
  key: keyof T;
  label: string;
  description?: string;
};

export type NumberControlField<T> = ControlField<T> & {
  step: number;
  min?: number;
  max?: number;
};
