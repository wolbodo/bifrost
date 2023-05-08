import type { Color } from "../type";
import { Base } from "./type";
import * as Solid from "./Solid.svelte";
import * as Fade from "./Fade.svelte";
import * as Blink from "./Blink.svelte";
import * as RandomChase from "./RandomChase.svelte";
import { invoke } from "@tauri-apps/api";
import { sequence } from "../engine";

const patterns = {
  blink: Blink,
  random_chase: RandomChase,
  solid: Solid,
  fade: Fade,
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
type Pattern = PatternTypes[keyof Patterns];

export const isPatternName = (name: string): name is keyof Patterns =>
  name in patterns;

const getPatternClass = (name: keyof Patterns): PatternMap[keyof Patterns] => {
  return Object.values(patterns[name]).find(
    (cls) => cls.prototype instanceof Base
  );
};

export const formatPattern = (
  pattern: Pattern
): { [name: string]: Pattern } => {
  const component = patterns[pattern.name].default;

  if (!component) {
    throw new Error("Unknown pattern");
  }
  const [, name] = component.name.match(new RegExp("Proxy<(.*)>"));
  return {
    [name]: pattern,
  };
};
export const getComponent = (pattern: Pattern) => {
  const component = patterns[pattern.name].default;

  if (!component) {
    throw new Error("Unknown pattern");
  }
  return component;
};

export const addPattern = (name: keyof PatternMap) => {
  const Cls = getPatternClass(name);
  const pattern = new Cls();

  invoke("add_pattern", { pattern: formatPattern(pattern) });
  sequence.update();
  // return new
};
