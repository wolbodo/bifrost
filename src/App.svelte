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
  <section>
    <span>Time: {$time}</span>
  </section>

  <section>
    <button on:click={() =>{t = 500; invoke('start_timer', { duration: t})}}>Play</button>
    <button on:click={() => { t += 500; invoke('set_timer', { duration: t})}}>up ({t})</button>

  </section>

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

  section {
    display: grid;
    grid-template-columns: auto min-content;
  }

  select {
    font-size: 1rem;
  }

  pre {
    grid-area: debug;
    border: thin solid black;
  }
</style>