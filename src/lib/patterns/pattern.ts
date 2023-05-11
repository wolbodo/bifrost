import type { Color } from "../type";
import { Base } from "./type";
import { invoke } from "@tauri-apps/api";
import { sequence } from "../engine";

import * as Solid from "./Solid.svelte";
import * as Fade from "./Fade.svelte";
import * as Blink from "./Blink.svelte";
import * as RandomChase from "./RandomChase.svelte";
import * as Wave from "./Wave.svelte";

export const patterns = {
  blink: Blink,
  random_chase: RandomChase,
  solid: Solid,
  fade: Fade,
  wave: Wave,
};

type Patterns = typeof patterns;
type PatternMap = {
  [K in keyof Patterns]: Patterns[K] extends { [key: string]: infer Imported }
    ? Imported extends typeof Base
      ? Imported
      : never
    : never;
};
type PatternTypes = {
  [K in keyof PatternMap]: InstanceType<PatternMap[K]>;
};
export type Pattern = PatternTypes[keyof Patterns];

export const isPatternName = (name: string): name is keyof Patterns =>
  name in patterns;

const getPatternClass = (name: keyof Patterns): PatternMap[keyof Patterns] => {
  return Object.values(patterns[name]).find(
    (cls) => cls.prototype instanceof Base
  );
};

export const packPattern = (pattern: Pattern): { [name: string]: Pattern } => {
  const module = patterns[pattern.name];

  if (!module) {
    throw new Error("Unknown pattern");
  }

  const [name] = Object.entries(module).find(
    ([, cls]) => cls.prototype instanceof Base
  );
  return {
    [name]: pattern,
  };
};
export const unpackPattern = (packed: {
  [name: string]: Pattern;
}): [name: string, pattern: Pattern] => {
  const [name, pattern] = Object.entries(packed)[0];
  return [name, pattern];
};
export const getComponent = (pattern: Pattern) => {
  const component = patterns[pattern.name].default;

  if (!component) {
    throw new Error("Unknown pattern");
  }
  return component;
};

export const addPattern = <K extends keyof PatternTypes>(
  name: K,
  values?: PatternTypes[K]
) => {
  const Cls = getPatternClass(name);
  const pattern = new Cls();

  if (values) {
    Object.assign(pattern, values);
  }

  invoke("add_pattern", { pattern: packPattern(pattern) });
  sequence.update();
};
