<script lang='ts' context='module'>
  import type { Color as TColor } from "../type";

  export type Blink = {
    name: "blink";
    color: TColor;
    on_duration: number;
    off_duration: number;
  };

</script>

<script lang=ts>
  import { createEventDispatcher } from 'svelte';
  import Color from 'color'

  export let data: Blink;

  const dispatch = createEventDispatcher();

  $: color = Color(data.color).hex()

  const onChange = async (diff: Partial<Blink>) => {
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
  min='1'
  max='100'
  value={data.on_duration}
  on:change={(e) => onChange({ on_duration: Number(e.target.value) })}
/>
<input
  type='range'
  min='1'
  max='100'
  value={data.off_duration}
  on:change={(e) => onChange({ off_duration: Number(e.target.value) })}
/>

<style>
  input {
    background: var(--color);

    width: 3rem;
    height: 3rem;
  }
</style>