<script lang='ts' context='module'>
  import type { Color as TColor } from "../type";

  export type RandomChase = {
    name: "random_chase";
    color: TColor;
    randomness: number;
    steps: number;
    speed: number;
  };

</script>

<script lang=ts>
  import { createEventDispatcher } from 'svelte';
  import Color from 'color'
  import Range from "../Range.svelte";
  import ColorPicker from "../ColorPicker.svelte";

  export let data: RandomChase;

  const dispatch = createEventDispatcher();

  const onChange = async (diff: Partial<RandomChase>) => {
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
  label='randomness'
  min='0'
  max='1'
  step='0.01'
  value={data.randomness}
  on:change={({ detail }) => {
    onChange({ randomness: Number(detail.value) })
  }}
/>
<Range
  label='steps'
  min='1'
  max='25'
  value={data.steps}
  on:change={({ detail }) => onChange({ steps: Number(detail.value) })}
/>
<Range
  label='speed'
  min='1'
  max='25'
  value={data.speed}
  on:change={({ detail }) => onChange({ speed: Number(detail.value) })}
/>

<style>
  input {
    background: var(--color);

    width: 3rem;
    height: 3rem;
  }
</style>