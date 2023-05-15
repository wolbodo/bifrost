<script lang="ts">
  import { sequence } from '../engine';
  import { invoke } from '@tauri-apps/api';

  import { debounce } from '../util';

  import { selected } from '../controls/Sequence.svelte'
  import { getComponent, packPattern, addPattern } from './pattern';
  import Range from '../controls/Range.svelte';

  $: selectedPattern = $sequence && $sequence.patterns[$selected]
  $: slot = $sequence && $sequence.track.find(({ id }) => $selected === id)

  const apply = debounce(async ({ detail }) => {
    console.log("apply", detail)
    await invoke("set_pattern",
      { index: $selected, pattern: packPattern(detail) }
    )
    sequence.update()
  }, 100)

  const deletePattern = async () => {
    invoke('delete_pattern', { index: $selected })
    sequence.update()

  }

  const clonePattern = () => {
    addPattern(selectedPattern.name, selectedPattern)
  }

  const setSpeed = (speed) => {
    slot.speed = speed

    console.log('track', $sequence.track)
    invoke('set_track', { track: $sequence.track })
    sequence.update()
  }

  $: { console.log('pattern', selectedPattern)}
</script>

{#if selectedPattern}

<section class='pattern'>
    <section>
      <button on:click={deletePattern}>Delete</button>
      <button on:click={clonePattern}>Clone</button>
    </section>
    <Range label='speed' min={0.01} max={4} step={0.01} value={slot.speed} on:change={(e) => setSpeed(parseFloat(e.detail.value))} />
    <svelte:component
      this={getComponent(selectedPattern)}
      on:change={apply}
      data={selectedPattern}
    />
    
  </section>
{/if}


<style>
  .pattern {
    grid-area: pattern;

    display: grid;
    padding: .5rem;
  }
</style>