<script lang='ts' context='module'>
  import {type Color as TColor, randomColor } from "../type";
  import { Base } from './type'

  export class Fade extends Base {
    name = "fade" as const;
    color: TColor;
    duration: number;
    fade_out: boolean;

    constructor(color?: TColor, duration = 1) {
      super()
      this.color = color ?? randomColor()
      this.duration = duration
      this.fade_out = false
    }
  };
</script>

<script lang=ts>
  import Color from 'color'
  import Range from "../controls/Range.svelte";
  import { createEventDispatcher } from 'svelte';
  import ColorPicker from "../controls/ColorPicker.svelte";
  import Checkbox from '../controls/Checkbox.svelte';

  export let data: Fade;

  const dispatch = createEventDispatcher();

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

  <Checkbox
    label='Fade out'
    checked={data.fade_out}
    on:change={({ detail }) => {
      onChange({ fade_out: detail.checked })
    }}
  />