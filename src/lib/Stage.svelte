<script lang='ts'>
  import { stage } from './engine';
</script>

<ul>
  {#each $stage?.rgb || [] as rgb}
    <li style="background: rgb({ rgb })" />
  {/each} 
</ul>

<style>
  ul {
    grid-area: stage;

    --size: 3rem;
    display: grid;
    grid-template-columns: repeat(4, minmax(var(--size), 1fr));
    gap: .2rem;
    list-style: none;
    margin: none;
  }

  li {
    width: 100%;
    aspect-ratio: 1;
  }
</style>