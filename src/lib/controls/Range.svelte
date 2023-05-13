<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();
  
  export let label: string;
  export let value: number;

  const onChange = (e: Event) => dispatch('change', { value: (e.target as HTMLInputElement).value })
  const presentValue = (value?: number): string => {
    if (value === undefined) return '';
    if ((value |0) === value) {
      return value.toString();
    }
    return value.toPrecision(2)
  }
</script>

<label>
  {label}: {presentValue(value)}
  <input
    {...$$restProps}
    type='range'
    bind:value
    on:change={onChange}
  />
</label>

<style>
  label {
    /* padding: .5rem; */
    margin: .2rem;
    box-sizing: border-box;
  }
  input[type=range] {
    -webkit-appearance: none;
    width: 100%;

    background-color: var(--primary-0);
  }
  input[type=range]:focus {
    outline: none;
  }
  input[type=range]::-webkit-slider-runnable-track {
    width: 100%;
    height: 8.4px;
    cursor: pointer;
    box-shadow: 1px 1px 1px var(--primary-4), 0px 0px 1px var(--primary-3);
    background: var(--primary-2);
    border-radius: 1.3px;
    border: 0.2px solid var(--primary-3);
  }
  input[type=range]::-webkit-slider-thumb {
    box-shadow: 1px 1px 1px var(--primary-4), 0px 0px 1px var(--primary-3);
    border: 1px solid var(--primary-4);
    height: 36px;
    width: 16px;
    border-radius: 3px;
    background: var(--primary-0);
    cursor: pointer;
    -webkit-appearance: none;
    margin-top: -14px;
  }
  input[type=range]:focus::-webkit-slider-runnable-track {
    background: var(--primary-1);
  }
  input[type=range]::-moz-range-track {
    width: 100%;
    height: 8.4px;
    cursor: pointer;
    box-shadow: 1px 1px 1px var(--primary-4), 0px 0px 1px var(--primary-3);
    background: var(--primary-2);
    border-radius: 1.3px;
    border: 0.2px solid var(--primary-2);
  } 
  input[type=range]::-moz-range-thumb {
    box-shadow: 1px 1px 1px var(--primary-4), 0px 0px 1px var(--primary-3);
    border: 1px solid var(--primary-4);
    height: 36px;
    width: 16px;
    border-radius: 3px;
    background: var(--primary-0);
    cursor: pointer;
  }
  input[type=range]::-ms-track {
    width: 100%;
    height: 8.4px;
    cursor: pointer;
    background: transparent;
    border-color: transparent;
    border-width: 16px 0;
    color: transparent;
  }
  input[type=range]::-ms-fill-lower {
    background: var(--primary-3);
    border: 0.2px solid var(--primary-3);
    border-radius: 2.6px;
    box-shadow: 1px 1px 1px var(--primary-4), 0px 0px 1px var(--primary-3);
  }
  input[type=range]::-ms-fill-upper {
    background: var(--primary-3);
    border: 0.2px solid var(--primary-3);
    border-radius: 2.6px;
    box-shadow: 1px 1px 1px var(--primary-4), 0px 0px 1px var(--primary-3);
  }
  input[type=range]::-ms-thumb {
    box-shadow: 1px 1px 1px var(--primary-4), 0px 0px 1px var(--primary-3);
    border: 1px solid var(--primary-4);
    height: 36px;
    width: 16px;
    border-radius: 3px;
    background: var(--primary-0);
    cursor: pointer;
  }
  input[type=range]:focus::-ms-fill-lower {
    background: var(--primary-3);
  }
  input[type=range]:focus::-ms-fill-upper {
    background: var(--primary-3);
  }
</style>