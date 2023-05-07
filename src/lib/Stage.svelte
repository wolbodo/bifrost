<script lang='ts'>
  import { stage } from './engine';
  import { discovery } from './mdns'
  import { invoke } from "@tauri-apps/api";

  let service

  $: if ($discovery && !service) {
    service = Object.keys($discovery)[0]
  }
  $: if (service) {
    invoke("set_service", { service });
  }

  $: {
    console.log($stage)
  }
</script>

<section>
  <select disabled={!$discovery} bind:value={service}>
    {#if $discovery}
      {#each Object.entries($discovery) as [name, service]}
        <option value={name}>{name}</option>
      {/each}
    {:else}
      <option value=''>loading services</option>
    {/if}
  </select>

  <ul>
    {#each $stage?.rgb || [] as rgb}
      <li style="background: rgb({ rgb })" />
    {/each} 
  </ul>
</section>

<style>
  section {
    grid-area: stage;
  }
  ul {
    --size: 2rem;
    display: grid;
    grid-template-columns: repeat(4, minmax(var(--size), 1fr));
    gap: .2rem;
    list-style: none;
    margin: none;

    border: thin solid var(--primary-color-1);
  }

  li {
    width: 100%;
    aspect-ratio: 1;
  }
</style>