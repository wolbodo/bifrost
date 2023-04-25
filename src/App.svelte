<script lang="ts">
  import { time, sequence }from './lib/engine'

  import Stage from './lib/Stage.svelte'
  import Sequence from './lib/Sequence.svelte'
  import Pattern from './lib/patterns/Pattern.svelte'

  $: currentPattern = $sequence?.patterns[$sequence.current]
</script>

<main>
  <span>Time: {$time}</span>

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

  span {
    grid-area: header;
  }

  pre {
    grid-area: debug;
    border: thin solid black;
  }
</style>