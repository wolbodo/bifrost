<script lang="ts">
  import { patterns } from '../engine';
  import { invoke } from '@tauri-apps/api';

  import { selected } from '../Sequence.svelte'
  import { type Pattern, getComponent, formatPattern } from './pattern';

  $: pattern = $patterns? $patterns[$selected] : null as Pattern

  const apply = async ({ detail }) => {
    console.log("apply", detail)
    await invoke("edit_pattern",
      { index: $selected, pattern: formatPattern(detail) }
    )
  }
</script>

{#if pattern}
  <section>
    <svelte:component
      this={getComponent(pattern)}
      on:change={apply}
      data={pattern}
    />
    
    <button on:click={() => invoke('delete_pattern', { index: $selected })}>Delete</button>
  </section>
{/if}


<style>
  section {
    grid-area: pattern;

    display: grid;
    
  }
</style>