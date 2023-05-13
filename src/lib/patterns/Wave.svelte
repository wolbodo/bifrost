<script lang='ts' context='module'>
  import {type Color as TColor, randomColor } from "../type";
  import { Base } from './type'

  export class Wave extends Base {
    name = "wave" as const;
    color: TColor;
    direction: boolean;
    size: number;

    constructor(color?: TColor) {
      super()
      this.color = color ?? randomColor()
      this.direction = false
      this.size = 1
    }
  };
</script>

<script lang=ts>
  import Color from 'color'
  import Range from "../controls/Range.svelte";
  import { createEventDispatcher } from 'svelte';
  import ColorPicker from "../controls/ColorPicker.svelte";
  import Checkbox from "../controls/Checkbox.svelte";

  export let data: Wave;

  const dispatch = createEventDispatcher();

  $: color = Color(data.color).hex()

  const onChange = async (diff: Partial<Wave>) => {
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
    label='size'
    min='0.1'
    max='4'
    step='0.1'
    value={data.size}
    on:change={({ detail }) => {
      onChange({ size: Number(detail.value) })
    }}
  />
  
  <Checkbox
    label='direction'
    checked={data.direction}
    on:change={({ detail }) => {
      onChange({ direction: detail.checked })
    }}
  />
  

<style>
  input {
    background: var(--color);

    width: 3rem;
    height: 3rem;
  }
</style>