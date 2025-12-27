export interface NumberData {
  numbers: number[];
  created_at?: string;
  updated_at?: string;
}

export interface NumberStats {
  count: number;
  sum: number;
  average: number;
  min: number;
  max: number;
  created_at?: string;
  updated_at?: string;
}