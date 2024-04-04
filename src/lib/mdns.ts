import { readable } from "svelte/store";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

type Service = {
  name: string;
  ip: string;
  port: number;
};

export const discovery = readable(null, (set) => {
  invoke("discover");
  listen<Record<string, Service>>("services", (event) => {
    set(event.payload);
  });
});
