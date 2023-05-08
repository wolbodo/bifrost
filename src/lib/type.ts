export type Color = [number, number, number];

export const randomColor = (): Color =>
  [0, 0, 0].map(() => Math.floor(Math.random() * 255)) as Color;
