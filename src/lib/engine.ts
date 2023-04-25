import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { derived, readable } from "svelte/store";

import type { Color } from "./type";
import type { Solid } from "./patterns/Solid.svelte";
import type { Fade } from "./patterns/Fade.svelte";
import type { Blink } from "./patterns/Blink.svelte";
import type { RandomChase } from "./patterns/RandomChase.svelte";

export type Pattern = Solid | Fade | Blink | RandomChase;

export const isPatternOf =
  <T extends Pattern>(name: Pattern["name"]) =>
  (pattern: Pattern): pattern is T =>
    pattern.name === name;

export const isSolidPattern = isPatternOf<Solid>("solid");
export const isFadePattern = isPatternOf<Fade>("fade");
export const isBlinkPattern = isPatternOf<Blink>("blink");
export const isRandomChasePattern = isPatternOf<RandomChase>("random_chase");

export type Sequence = {
  current: number;
  patterns: Pattern[];
  time: number;
};

export type Stage = {
  rgb: [number, number, number][];
  size: number;
};

export type Engine = {
  sequence: Sequence;
  speed: number;
  stage: Stage;
};

export const engine = readable<Engine>(null, (set) => {
  invoke("init_engine");
  listen<Engine>("tick", (event) => {
    set(event.payload);
  });
});

export const time = derived(engine, (engine) => engine?.sequence.time);
export const stage = derived(engine, (engine) => engine?.stage);
export const currentPattern = derived(
  engine,
  (engine) => engine?.sequence.current
);
export const sequence = derived(engine, (engine) => engine?.sequence);
export const patterns = derived(
  engine,
  (engine) => engine?.sequence.patterns || []
);
