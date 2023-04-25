<script lang='ts' context='module'>
  import Color from 'color'

  export type Solid = {
    name: "solid";
    color: TColor;
    on_duration: number;
    off_duration: number;
  };
</script>

<script lang=ts>
  import type { Color as TColor } from "../type";
  import { createEventDispatcher } from 'svelte';

  export let data: Solid;

  const dispatch = createEventDispatcher();

  $: color = Color(data.color).hex()

  const onChange = async (e: Event) => {
    const target = e.target as HTMLInputElement
    color = Color(target.value)

    dispatch('change', { ...data, color: color.rgb().array()})
  }   
</script>

<section>
  <h2>{data.name}</h2>
  <input
    type='color'
    style:--color={color}
    value={color}
    on:change={onChange}
  />
</section>

<style>
  section {
    grid-area: pattern;
  }
  input {
    background: var(--color);

    width: 3rem;
    height: 3rem;
  }
</style>