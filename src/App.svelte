<script lang="ts">
  import { time, sequence } from "./lib/engine";

  import { invoke } from "@tauri-apps/api/core";
  import Stage from "./lib/controls/Stage.svelte";
  import Sequence from "./lib/controls/Sequence.svelte";
  import Pattern from "./lib/patterns/Pattern.svelte";
</script>

<main>
  <header>
    <span>Time: {$time}</span>
    <button on:click={() => invoke("start_engine")}>▶️</button>
    <button on:click={() => invoke("stop_engine")}>⏹️</button>
    <button on:click={() => invoke("set_bpm", { bpm: 30 })}>30</button>
    <button on:click={() => invoke("set_bpm", { bpm: 60 })}>60</button>
    <button on:click={() => invoke("set_bpm", { bpm: 120 })}>120</button>
    <button on:click={() => invoke("set_bpm", { bpm: 240 })}>240</button>
    <button on:click={() => invoke("clear")}>Clear</button>
  </header>

  <Sequence />
  <Pattern />

  <Stage />
</main>

<style>
  main {
    display: grid;
    grid:
      "header header" 2.5rem
      "sequence sequence" min-content
      "pattern stage" auto
      "debug debug" auto / 1fr 1fr;

    max-height: 100vh;
  }

  header {
    grid-area: header;
    grid: 2.5rem / auto repeat(7, min-content);
    gap: 0.5rem;
    display: grid;
  }
  header span {
    display: inline-block;
  }

  header button {
    aspect-ratio: 1;
    width: 2rem;
    padding: 0;
  }

  pre {
    grid-area: debug;
    border: thin solid black;
  }
</style>
