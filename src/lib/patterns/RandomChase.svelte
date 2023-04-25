<script lang='ts' context='module'>
  import type { Color as TColor } from "../type";

  export type RandomChase = {
    name: "random_chase";
    color: TColor;
    randomness: number;
    steps: number;
    speed: number;
    // on_duration: number;
    // off_duration: number;
  };

</script>

<script lang=ts>
  import { createEventDispatcher } from 'svelte';
  import Color from 'color'

  export let data: RandomChase;

  const dispatch = createEventDispatcher();

  $: color = Color(data.color).hex()

  const onChange = async (diff: Partial<RandomChase>) => {
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
<input
  type='range'
  min='0'
  max='1'
  step='0.01'
  value={data.randomness}
  on:change={(e) => onChange({ randomness: Number(e.target.value) })}
/>
<input
  type='range'
  min='1'
  max='25'
  value={data.steps}
  on:change={(e) => onChange({ steps: Number(e.target.value) })}
/>

<style>
  input {
    background: var(--color);

    width: 3rem;
    height: 3rem;
  }
</style>