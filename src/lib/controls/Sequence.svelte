<script lang="ts" context="module">
  import { writable } from "svelte/store";

  export const selected = writable<number>(0);
</script>

<script lang="ts">
  import { sequence } from "../engine";
  import { patterns, addPattern, isPatternName } from "../patterns/pattern";
  import Track from "./Track.svelte";
  import { invoke } from "@tauri-apps/api/core";

  const onChange = (e: any) => {
    const name: string = e.target.value;
    const reset = (e.target.value = e.target.options[0].value);

    if (!isPatternName(name)) return;

    addPattern(name);
  };

  $: if (
    $sequence &&
    !($selected in $sequence.patterns) &&
    $sequence.track.length > 0
  ) {
    const last = $sequence.track[$sequence.track.length - 1];
    selected.set(last.id);
  }
</script>

<div>
  <select on:change={onChange}>
    <option>Add +</option>
    {#each Object.keys(patterns) as name}
      <option>{name}</option>
    {/each}
  </select>

  {#if $sequence}
    <Track
      items={$sequence.track}
      let:item
      on:change={({ detail: items }) => {
        console.log("track", items);
        invoke("set_track", { track: items });
        sequence.update();
      }}
    >
      {@const name = $sequence.patterns[item.id].type}
      <span
        class:selected={$selected === item.id}
        on:click|preventDefault={() => {
          $selected = item.id;
        }}
      >
        {name}
      </span>
    </Track>
  {/if}
</div>

<style>
  div {
    grid-area: sequence;
    border-top: thin solid var(--primary-1);
    border-bottom: thin solid var(--primary-1);
    padding: 0.5rem;
  }
  section {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(4rem, 1fr));
    gap: 0.5rem;
    height: 4rem;
    margin: 0.5rem;
  }

  span {
    width: 100%;
    height: 100%;
    display: block;
  }

  button {
    padding: 0.5rem;
    line-height: calc(var(--size) - 2 * 0.5rem);
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
