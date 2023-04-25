<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();
  
  export let label: string;
  export let value: number;
</script>

<label>
  {label}:
  <input
    {...$$restProps}
    type='range'
    {value}
    on:change={(e) => dispatch('change', { value: e.target.value })}
  />
</label>

<style>
  input {
    display: block;
    width: 100%;
  }
</style>