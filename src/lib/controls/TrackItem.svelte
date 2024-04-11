<script lang="ts">
  import { onMount } from "svelte";

  export let width = 1;
  export let observer: ResizeObserver;
  export let mouseMove;
  export let startDrag;
  export let handleKeyDown;

  let element: HTMLElement;

  onMount(() => {
    if (element && observer) {
      console.log("observe", element);
      observer.observe(element);
    }
  });
</script>

<section
  bind:this={element}
  style:grid-column-start="span {width}"
  aria-label="drag-handle"
  class="handle"
  on:mousemove={mouseMove}
  on:mousedown={startDrag}
  on:touchstart={startDrag}
  on:keydown={handleKeyDown}
>
  <slot />
</section>

<style>
  section {
    grid-column-end: auto;
    box-sizing: border-box;
    background: var(--primary-2);
    text-align: center;
    line-height: 2rem;

    overflow: hidden;
    resize: horizontal;
  }
</style>
