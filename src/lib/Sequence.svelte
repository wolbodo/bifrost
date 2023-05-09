<script lang="ts" context="module">
  import { writable } from "svelte/store";

  export const selected = writable<number>(0)
</script>

<script lang="ts">
  import { sequence } from "./engine";
  import { addPattern, isPatternName } from "./patterns/pattern";
  import Track from "./Track.svelte";
  import { invoke } from "@tauri-apps/api";

  const select = (index) => $selected = index;
  const onChange = (e) => {
    const name: string = e.target.value;
    const reset = e.target.value = e.target.options[0].value

    if (!isPatternName(name)) return;

    addPattern(name)
  }

  
</script>

<div>
  <select on:change={onChange}>
    <option>Add +</option>
    {#each ['solid', 'blink', 'fade', 'random_chase'] as name}
    <option>{name}</option>
    {/each}
  </select>

  {#if $sequence}
    <Track items={$sequence.track} let:item on:change={({ detail: items }) => {
      console.log('track', items)
      invoke('set_track', { track: items })
      sequence.update()
    }}>
      <span>{item.id}</span>
    </Track>
  {/if}
  
  <!-- <section>
    {#each $patterns || [] as pattern, index}
    <button
    class:current={$sequence.current === index}
    class:selected={$selected === index}
      on:click={() => select(index)}
    >
      {pattern.name}
    </button>
    {/each}
  </section> -->
</div>

<style>
  div {
    grid-area: sequence;
    border-top: thin solid var(--primary-1);
    border-bottom: thin solid var(--primary-1);
    padding: .5rem;
  }
  section {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(4rem, 1fr));
    gap: .5rem;
    height: 4rem;
    margin: .5rem;
  }

  button {
    padding: .5rem;
    line-height: calc(var(--size) - 2*.5rem);
    text-align: center;
    background: var(--primary-4);
    color: var(--primary-2);
    font-weight: bold;
    border-radius: 0;

    cursor: pointer;
    box-sizing: border-box;
  }

  .current {
    outline: 5px solid var(--primary-2);
  }
  .selected {
    background: var(--primary-3);
  }

</style>