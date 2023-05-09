<script lang="ts">
  import { sequence } from '../engine';
  import { invoke } from '@tauri-apps/api';

  import { debounce } from '../util';

  import { selected } from '../Sequence.svelte'
  import { type Pattern, getComponent, formatPattern } from './pattern';

  $: pattern = $sequence? $sequence.patterns[$selected] : null as Pattern

  const apply = debounce(async ({ detail }) => {
    console.log("apply", detail)
    await invoke("edit_pattern",
      { index: $selected, pattern: formatPattern(detail) }
    )
    sequence.update()
  }, 100)

  const deletePattern = () => {
    invoke('delete_pattern', { index: $selected })
    sequence.update()
  }
</script>

{#if pattern}
  <section>
    <svelte:component
      this={getComponent(pattern)}
      on:change={apply}
      data={pattern}
    />
    
    <button on:click={deletePattern}>Delete</button>
  </section>
{/if}


<style>
  section {
    grid-area: pattern;

    display: grid;
    
  }
</style>