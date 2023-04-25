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

  export let data: RandomChase;

  const dispatch = createEventDispatcher();

  $: color = Color(data.color).hex()

  const onChange = async (diff: Partial<RandomChase>) => {
    console.log("change", diff)
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
  label='randomness'
  min='0'
  max='1'
  step='0.01'
  value={data.randomness}
  on:change={({ detail }) => {
    console.log('hmm');
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