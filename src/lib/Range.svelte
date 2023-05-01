<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();
  
  export let label: string;
  export let value: number;

  const onChange = (e: Event) => dispatch('change', { value: (e.target as HTMLInputElement).value })
</script>

<label>
  {label}: {value}
  <input
    {...$$restProps}
    type='range'
    {value}
    on:change={onChange}
  />
</label>

<style>
  input {
    display: block;
    width: 100%;
  }
</style>