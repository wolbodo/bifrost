<script lang='ts' context='module'>
  import Color from 'color'

  export type Solid = {
    name: "solid";
    color: TColor;
    on_duration: number;
    off_duration: number;
  };
</script>

<script lang=ts>
  import type { Color as TColor } from "../type";
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
