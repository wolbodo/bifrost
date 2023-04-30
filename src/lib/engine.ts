import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { derived, readable } from "svelte/store";

import type { Pattern } from "./patterns/pattern";

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
