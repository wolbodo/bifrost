<script lang='ts' context='module'>
  import { patterns, type Pattern, isSolidPattern, isFadePattern, isBlinkPattern } from '../engine';
  import Solid, { type Solid as TSolid } from './Solid.svelte'
  import Fade, { type Fade as TFade } from './Fade.svelte'
  import Blink, { type Blink as TBlink } from './Blink.svelte'

  export type PatternEnum = {
    Solid: TSolid,
  } | {
    Fade: TFade,
  } | {
    Blink: TBlink,
  }
  
  export const formatPattern = (pattern: Pattern): PatternEnum => {
    if (isSolidPattern(pattern)) {
      return { Solid: pattern }
    } else if (isFadePattern(pattern)) {
      return { Fade: pattern }
    } else if (isBlinkPattern(pattern)) {
      return { Blink: pattern }
    } else {
      throw new Error("Unknown pattern")
    }
  }
</script>

<script lang="ts">
  import { invoke } from '@tauri-apps/api';

  import { selected } from '../Sequence.svelte'

  $: pattern = $patterns? $patterns[$selected] : null as Pattern

  const apply = async ({ detail }) => {
    console.log("apply", detail)
    await invoke("edit_pattern",
      { index: $selected, pattern: formatPattern(detail) }
    )
  }
</script>

{#if pattern}
  {#if isSolidPattern(pattern)}
    <Solid data={pattern} on:change={apply} />
  {:else if isFadePattern(pattern)}
    <Fade data={pattern} on:change={apply} />
  {:else if isBlinkPattern(pattern)}
    <Blink data={pattern} on:change={apply} />
  {:else}
    <p>Unknown pattern</p>
  {/if}
{/if}
