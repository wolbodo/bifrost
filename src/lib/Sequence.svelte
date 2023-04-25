<script lang="ts" context="module">
  import { writable } from "svelte/store";

  export const selected = writable<number>(0)
</script>

<script lang="ts">
  import { sequence } from "./engine";
  import { invoke } from "@tauri-apps/api";
  import { formatPattern } from "./patterns/Pattern.svelte";
  import type { Pattern } from "./engine";
  import type { Solid} from "./patterns/Solid.svelte";
  import type { Blink } from "./patterns/Blink.svelte";
  import type { Fade } from "./patterns/Fade.svelte";

  const select = (index) => $selected = index;

  const addPattern = (pattern: Pattern): void => {
    invoke("add_pattern", { pattern: formatPattern(pattern) })
  }

  const onChange = (e) => {
    const name: string = e.target.value;
    const reset = e.target.value = e.target.options[0].value

    if (name === reset) return;

    if (name === 'solid') {
      addPattern({
        name,
        color: [255, 255, 255],
      } as Solid)
    } else if (name === 'blink') {
      addPattern({
        name,
        color: [255, 255, 255],
        on_duration: 10,
        off_duration: 10,
      } as Blink)
    } else if (name === 'fade') {
      addPattern({
        name,
        color: [255, 255, 255],
        duration: 500,
      } as Fade)
    } else {
      throw new Error(`Unknown pattern name: ${name}`)
    }
    
  }

</script>

<div>

  <select on:change={onChange}>
    <option>Add +</option>
    {#each ['solid', 'blink', 'fade'] as name}
    <option>{name}</option>
    {/each}
  </select>
  
  <section>
    {#each $sequence?.patterns || [] as pattern, index}
    <button
    class:current={$sequence.current === index}
    class:selected={$selected === index}
      on:click={() => select(index)}
    >
      {pattern.name}
    </button>
    {/each}
    
  </section>
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