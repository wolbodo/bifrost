import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { derived, readable, writable } from "svelte/store";

import type { Pattern } from "./patterns/pattern";

export type Sequence = {
  patterns: { [id: string]: Pattern }[];
  track: {
    id: number;
    width: number;
    speed: number;
  }[];
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
  invoke<Engine>("get_engine").then((engine) => {
    console.log(engine);
    set(engine);
  });
});
export const stage = readable<Stage>(null, (set) => {
  listen<Stage>("tick", (event) => {
    set(event.payload);
  });
});
export const sequence = (() => {
  const update = () =>
    invoke<Sequence>("get_sequence").then((s) => {
      console.log("new sequence", s);
      seq.set(s);
    });
  const seq = writable<Sequence>(null, () => {
    update();
  });

  return {
    subscribe: seq.subscribe,
    update,
  };
})();

export const time = derived(engine, (engine) => engine?.sequence.time);
export const currentPattern = derived(
  engine,
  (engine) => engine?.sequence.current
);
