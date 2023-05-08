<script lang='ts' context='module'>
  import Color from 'color'
  import {type Color as TColor, randomColor } from "../type";

  import { Base } from './type'

  export class Solid extends Base {
    name = "solid" as const;
    color: TColor;

    constructor(color?: TColor) {
      super()
      this.color = color ?? randomColor()
    }
  };

</script>

<script lang=ts>
  import { createEventDispatcher } from 'svelte';
  import ColorPicker from '../ColorPicker.svelte';

  export let data: Solid;

  const dispatch = createEventDispatcher();

  const onChange = async (diff: Partial<Solid>) => {
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
