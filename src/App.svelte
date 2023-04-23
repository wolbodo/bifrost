<script lang="ts">
  import { engine }from './lib/engine'
  import Stage from './lib/Stage.svelte'

  $: console.log($engine)

  $: currentPattern = $engine?.sequence?.patterns[$engine.sequence.current]
</script>

{#if $engine}
<main>
  <span>Time: {$engine.sequence.time}</span>

  <ul>
    {#each $engine.sequence.patterns as pattern, index}
      <li class:current={$engine.sequence.current === index}>
        {pattern.name}
      </li>
    {/each}
  </ul>

  <pre>{JSON.stringify(currentPattern, null, 2)}</pre>
  
  <Stage stage={$engine.stage} />
</main>
{/if}

<style>
  .current {
    border: thin solid black;
  }
  pre {
    border: thin solid black;
  }
</style>