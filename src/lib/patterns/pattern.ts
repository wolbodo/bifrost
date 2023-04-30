import type { Color } from "../type";
import * as Solid from "./Solid.svelte";
import * as Fade from "./Fade.svelte";
import * as Blink from "./Blink.svelte";
import * as RandomChase from "./RandomChase.svelte";

export type Pattern =
  | Solid.Solid
  | Fade.Fade
  | Blink.Blink
  | RandomChase.RandomChase;

const patternMap = {
  solid: Solid,
  fade: Fade,
  blink: Blink,
  random_chase: RandomChase,
};

export const isPatternOf =
  <T extends Pattern>(name: Pattern["name"]) =>
  (pattern: Pattern): pattern is T =>
    pattern.name === name;

export const isSolidPattern = isPatternOf<Solid.Solid>("solid");
export const isFadePattern = isPatternOf<Fade.Fade>("fade");
export const isBlinkPattern = isPatternOf<Blink.Blink>("blink");
export const isRandomChasePattern =
  isPatternOf<RandomChase.RandomChase>("random_chase");

export const formatPattern = (
  pattern: Pattern
): { [name: string]: Pattern } => {
  const component = patternMap[pattern.name];

  if (!component) {
    throw new Error("Unknown pattern");
  }
  const [, name] = component.default.name.match(new RegExp("Proxy<(.*)>"));
  return {
    [name]: pattern,
  };
};
export const getComponent = (pattern: Pattern) => {
  const component = patternMap[pattern.name];

  if (!component) {
    throw new Error("Unknown pattern");
  }
  return component.default;
};
