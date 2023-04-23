import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { readable } from "svelte/store";

type Color = [number, number, number];
type Fade = {
  name: "fade";
  color: Color;
  duration: number;
};
type Blink = {
  name: "blink";
  color: Color;
  on_duration: number;
  off_duration: number;
};
type Solid = {
  name: "solid";
  color: Color;
  on_duration: number;
  off_duration: number;
};

export type Pattern = Solid | Fade | Blink;

export const isSolidPattern = (pattern: Pattern): pattern is Solid =>
  pattern.name === "solid";
export const isFadePattern = (pattern: Pattern): pattern is Fade =>
  pattern.name === "fade";
export const isBlinkPattern = (pattern: Pattern): pattern is Blink =>
  pattern.name === "blink";

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
