<script lang='ts' context='module'>
  import { Base } from './type'

  export class RainbowWave extends Base {
    name = "rainbow_wave" as const;
    saturation: number;
    lightness: number;
    direction: boolean;
    size: number;

    constructor(color?: TColor) {
      super()
      this.direction = false
      this.saturation = 1;
      this.lightness = 0.5;
      this.size = 1
    }
  };
</script>

<script lang=ts>
  import Color from 'color'
  import Range from "../Range.svelte";
  import { createEventDispatcher } from 'svelte';
  import ColorPicker from "../ColorPicker.svelte";

  export let data: RainbowWave;

  const dispatch = createEventDispatcher();


  const onChange = async (diff: Partial<RainbowWave>) => {
    dispatch('change', { ...data, ...diff })
  }   
</script>

  <h2>{data.name}</h2>
  
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

  <label>Fade out<input type='checkbox' checked={data.direction} on:change={({ target }) => {
    onChange({ direction: target.checked })
  }}/></label>

<style>
  input {
    background: var(--color);

    width: 3rem;
    height: 3rem;
  }
</style>