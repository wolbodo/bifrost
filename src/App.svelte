<script lang="ts">
  import { time, sequence }from './lib/engine'
  import { discovery } from './lib/mdns'

  import Stage from './lib/Stage.svelte'
  import Sequence from './lib/Sequence.svelte'
  import Pattern from './lib/patterns/Pattern.svelte'

  $: currentPattern = $sequence?.patterns[$sequence.current]

  $: console.log($discovery)
</script>

<main>
  <section>
    <span>Time: {$time}</span>

    <select disabled={!$discovery}>
      {#if $discovery}
        {#each Object.entries($discovery) as [name, service]}
          <option value={name}>{name}</option>
        {/each}
      {:else}
        <option value="none">loading</option>
      {/if}
    </select>
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