<script lang="ts">
  import { output } from "../engine";
  import { discovery } from "../mdns";
  // import { invoke } from "@tauri-apps/api/core";

  // let service;

  // $: if ($discovery && !service) {
  //   service = Object.keys($discovery)[0];
  // }
  // $: if (service) {
  //   invoke("set_service", { service });
  // }

  $: {
    console.log("output", $output);
  }
</script>

<section>
  <!-- <select disabled={!$discovery} bind:value={service}>
    {#if $discovery}
      {#each Object.entries($discovery) as [name, service]}
        <option value={name}>{name}</option>
      {/each}
    {:else}
      <option value="">loading services</option>
    {/if}
  </select> -->

  <ul style:--width={$output?.width}>
    {#each $output?.buffer || [] as rgb}
      <li style="background: rgb({rgb})" />
    {/each}
  </ul>
</section>

<style>
  section {
    grid-area: output;
  }
  ul {
    --size: 0.5rem;
    display: grid;
    grid-template-columns: repeat(var(--width, 4), minmax(var(--size), 1fr));
    max-height: 10rem;
    gap: 0.2rem;
    list-style: none;
    margin: none;

    border: thin solid var(--primary-color-1);
  }

  li {
    width: 100%;
    aspect-ratio: 1;
  }
</style>
