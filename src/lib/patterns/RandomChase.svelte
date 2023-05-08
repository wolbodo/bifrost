<script lang='ts' context='module'>
  import {type Color as TColor, randomColor } from "../type";
  import { Base } from './type'

  export class RandomChase extends Base {
    name = "random_chase" as const;
    color: TColor;
    randomness: number;
    steps: number;
    speed: number;
    
    constructor(color?: TColor, randomness = 0.1, steps = 4, speed = 1) {
      super()
      this.color = color ?? randomColor()
      this.randomness = randomness
      this.steps = steps;
      this.speed = speed;
    }
  }

  // type T = RandomChase['name']

  //   constructor() {
  //     this.name = "random_chase";
  //     this.color = [0, 0, 0];
  //     this.randomness = 0.5;
  //     this.steps = 10;
  //     this.speed = 0.5;
  //   }
  // };

</script>

<script lang=ts>
  import { createEventDispatcher } from 'svelte';
  import Color from 'color'
  import Range from "../Range.svelte";
  import ColorPicker from "../ColorPicker.svelte";

  export let data: RandomChase;

  const dispatch = createEventDispatcher();

  const onChange = async (diff: Partial<RandomChase>) => {
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
  label='randomness'
  min='0'
  max='1'
  step='0.01'
  value={data.randomness}
  on:change={({ detail }) => {
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