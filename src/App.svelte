<script lang="ts">
  import { time, sequence }from './lib/engine'
  
  import { invoke } from '@tauri-apps/api'
  import Stage from './lib/Stage.svelte'
  import Sequence from './lib/Sequence.svelte'
  import Pattern from './lib/patterns/Pattern.svelte'

  $: currentPattern = $sequence?.patterns[$sequence.current]

  let t = 500
</script>

<main>
  <header>
    <span>Time: {$time}</span>
    <button on:click={() => invoke('start_engine')}>▶️</button>
    <button on:click={() => invoke('stop_engine')}>⏹️</button>
    <button on:click={() => invoke('set_bpm', { bpm: 240 })}>🔄</button>
  </header>

  <Sequence />
  <Pattern />

  <pre>{JSON.stringify(currentPattern, null, 2)}</pre>
  
  <Stage/>
</main>

<style>
  main {
    display: grid;
    grid-template-areas:
      "header header"
      "sequence sequence"
      "pattern stage"
      "debug debug";
  }

  header {
    grid-area: header;
    grid: 2.5rem / auto repeat(3, min-content);
    gap: .5rem;
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