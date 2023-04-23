<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { onMount } from "svelte";

  let name = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name })
  }

  $: {
    console.log("HII")
    invoke("get_sequence").then((msg) => {
      console.log(msg)
    })
  }
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}>
      Greet
    </button>
  </div>
  <p>{greetMsg}</p>
</div>