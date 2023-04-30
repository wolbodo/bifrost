<script lang='ts' context='module'>
  import type { Color as TColor } from "../type";

  export type Fade = {
    name: "fade";
    color: TColor;
    duration: number;
  };
</script>

<script lang=ts>
  import Color from 'color'
  import Range from "../Range.svelte";
  import { createEventDispatcher } from 'svelte';

  export let data: Fade;

  const dispatch = createEventDispatcher();

  $: color = Color(data.color).hex()

  const onChange = async (diff: Partial<Fade>) => {
    dispatch('change', { ...data, ...diff })
  }   
</script>

  <h2>{data.name}</h2>
  <input
    type='color'
    style:--color={color}
    value={color}
    on:change={(e) => onChange({ color: Color(e.target.value).rgb().array() })}
  />
  <Range
    label='duration'
    min='0'
    max='100'
    value={data.duration}
    on:change={({ detail }) => {
      onChange({ duration: Number(detail.value) })
    }}
  />

<style>
  input {
    background: var(--color);

    width: 3rem;
    height: 3rem;
  }
</style>