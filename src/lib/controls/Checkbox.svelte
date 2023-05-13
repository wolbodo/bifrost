<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();
  
  export let label: string;
  export let checked: boolean;

  const onChange = (e: Event) => dispatch('change', { checked: (e.target as HTMLInputElement).checked })
</script>


<label>
  <input 
    {...$$restProps}
    type='checkbox'
    {checked}
    on:change={onChange}
  >
  <div class="toggle-switch"></div>
  <span class="toggle-label">{label}</span>
</label>

<style>
  
  
label {
  cursor: pointer;
  display: inline-block;
}

div {
  display: inline-block;
  background: var(--primary-3);
  border-radius: 16px;
  width: 58px;
  height: 32px;
  position: relative;
  vertical-align: middle;
  transition: background 0.25s;
}
div:before, div:after {
  content: "";
}
div:before {
  display: block;
  background: linear-gradient(to bottom, var(--primary-0) 0%, var(--primary-2) 100%);
  border-radius: 50%;
  box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.25);
  width: 24px;
  height: 24px;
  position: absolute;
  top: 4px;
  left: 4px;
  transition: left 0.25s;
}
label:hover div:before {
  background: linear-gradient(to bottom, var(--primary-0) 0%, var(--primary-1) 100%);
  box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.5);
}
input:checked + div {
  background: var(--primary-1);
}
input:checked + div:before {
  left: 30px;
}

input {
  position: absolute;
  visibility: hidden;
}

span {
  margin-left: 5px;
  position: relative;
  top: 2px;
}
</style>