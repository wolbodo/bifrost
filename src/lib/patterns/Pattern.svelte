<script lang="ts">
  import { sequence } from '../engine';
  import { invoke } from '@tauri-apps/api';

  import { debounce } from '../util';

  import { selected } from '../Sequence.svelte'
  import { type Pattern, getComponent, packPattern, unpackPattern } from './pattern';

  $: selectedPattern = $sequence && $sequence.patterns[$selected]
  $: ([, pattern] = selectedPattern ? unpackPattern(selectedPattern) : [])

  const apply = debounce(async ({ detail }) => {
    console.log("apply", detail)
    await invoke("set_pattern",
      { index: $selected, pattern: packPattern(detail) }
    )
    sequence.update()
  }, 100)

  const deletePattern = () => {
    invoke('delete_pattern', { index: $selected })
    sequence.update()
  }

  $: { console.log('pattern', pattern)}
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