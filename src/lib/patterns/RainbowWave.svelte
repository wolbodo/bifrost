<script lang='ts' context='module'>
  import { Base } from './type'

  export class RainbowWave extends Base {
    name = "rainbow_wave" as const;
    saturation: number;
    lightness: number;
    direction: boolean;
    size: number;

    constructor() {
      super()
      this.direction = false
      this.saturation = 1;
      this.lightness = 0.5;
      this.size = 1
    }
  };
</script>

<script lang=ts>
  import Range from "../controls/Range.svelte";
  import { createEventDispatcher } from 'svelte';
  import Checkbox from '../controls/Checkbox.svelte';

  export let data: RainbowWave;

  const dispatch = createEventDispatcher();


  const onChange = async (diff: Partial<RainbowWave>) => {
    dispatch('change', { ...data, ...diff })
  }   
</script>

  <h2>{data.name}</h2>
  
  <Range
    label='saturation'
    min='0'
    max='1'
    step='0.05'
    value={data.saturation}
    on:change={({ detail }) => {
      onChange({ saturation: Number(detail.value) })
    }}
  />
  <Range
    label='lightness'
    min='0'
    max='1'
    step='0.05'
    value={data.lightness}
    on:change={({ detail }) => {
      onChange({ lightness: Number(detail.value) })
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