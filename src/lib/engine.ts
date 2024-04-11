import { invoke } from "@tauri-apps/api/core";

import { listen } from "@tauri-apps/api/event";
import { derived, readable, writable } from "svelte/store";

import type { Pattern } from "./patterns/pattern";

export type Sequence = {
  patterns: Pattern[];
  track: {
    id: number;
    width: number;
    speed: number;
  }[];
  time: number;
};

export type Output = {
  buffer: [number, number, number][];
  height: number;
  width: number;
};

export type Engine = {
  sequence: Sequence;
  speed: number;
  stage: Output;
};

export const engine = readable<Engine>(null, (set) => {
  invoke<Engine>("get_engine").then((engine) => {
    console.log(engine);
    set(engine);
  });
});
export const output = readable<Output>(null, (set) => {
  listen<Output>("output", (event) => {
    set(event.payload);
  });
});
export const time = readable<number>(0, (set) => {
  listen<number>("tick", (event) => {
    set(event.payload);
  });
});
export const sequence = (() => {
  const update = async () => seq.set(await invoke<Sequence>("get_sequence"));
  const seq = writable<Sequence>(null, () => update());

  return {
    subscribe: seq.subscribe,
    update,
  };
})();

export const currentPattern = derived(
  engine,
  (engine) => engine?.sequence.current
);
