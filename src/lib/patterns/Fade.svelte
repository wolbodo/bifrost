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
  import ColorPicker from "../ColorPicker.svelte";

  export let data: Fade;

  const dispatch = createEventDispatcher();

  $: color = Color(data.color).hex()

  const onChange = async (diff: Partial<Fade>) => {
    dispatch('change', { ...data, ...diff })
  }   
</script>

  <h2>{data.name}</h2>
  
  <ColorPicker
    color={Color.rgb(data.color)}
    on:change={({ detail }) => {
      onChange({ color: detail.value })
    }}
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