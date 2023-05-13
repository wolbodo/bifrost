<script lang='ts' context='module'>
  import {type Color as TColor, randomColor } from "../type";
  import { Base } from './type'

  export class Blink extends Base {
    name = "blink" as const;
    color: TColor;
    on: number;

    constructor(color?: TColor, on = 0.5) {
      super()
      this.color = color ?? randomColor()
      this.on = on
    }
  };

</script>

<script lang=ts>
  import { createEventDispatcher } from 'svelte';
  import Color from 'color'
  import Range from "../controls/Range.svelte";
  import ColorPicker from "../controls/ColorPicker.svelte";

  export let data: Blink;

  const dispatch = createEventDispatcher();

  $: color = Color(data.color)

  const onChange = async (diff: Partial<Blink>) => {
    dispatch('change', { ...data, ...diff })
  }   
</script>

<h2>{data.name}</h2>

<ColorPicker
  {color}
  on:change={({ detail }) => {
    onChange({ color: detail.value })
  }}
/>

<Range
  label='on'
  min='0'
  max='1'
  step='0.01'
  value={data.on}
  on:change={({ detail: { value }}) => onChange({ on: Number(value) })}
/>
