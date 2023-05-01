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
  import Range from "../Range.svelte";
  import ColorPicker from "../ColorPicker.svelte";

  export let data: Blink;

  const dispatch = createEventDispatcher();

  $: color = Color(data.color).hex()

  const onChange = async (diff: Partial<Blink>) => {
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
  label='on'
  min='1'
  max='100'
  value={data.on_duration}
  on:change={({ detail: { value }}) => onChange({ on_duration: Number(value) })}
/>
<Range
  label='off'
  min='1'
  max='100'
  value={data.off_duration}
  on:change={({ detail: { value }}) => onChange({ off_duration: Number(value) })}
/>
