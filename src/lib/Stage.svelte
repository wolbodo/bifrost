<script lang='ts'>
  import type { Stage } from './engine'
  
  export let stage: Stage
</script>

<ul>
  {#each stage.rgb as rgb}
    <li style="background: rgb({ rgb })" />
  {/each}
</ul>

<style>
  ul {
    --size: 3rem;
    display: grid;
    grid-template-columns: repeat(4, var(--size));
    gap: .2rem;
    list-style: none;
    margin: none;
  }

  li {
    width: var(--size);
    height: var(--size);
  }
</style>