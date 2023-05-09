<script lang='ts'>
  import { flip } from 'svelte/animate';
  import { dndzone, SOURCES, TRIGGERS	 } from 'svelte-dnd-action';
  
  const GRID_SPACING = 16;
  const flipDurationMs = 300;

  interface ItemInterface {
    id: number | string;
    width: number;
  }
  type Item = $$Generic<ItemInterface>
  
  interface $$Slots {
    default: {
      item: Item
    }
  }

  export let items: Item[] = []
  let dragDisabled = true
  let element: HTMLElement;

  // Options for the observer (which mutations to observe)
  const config = { attributes: true, childList: true, subtree: true };

  $: if (element) observer.observe(element, config)

  function handleConsider(e) {
    const {items: newItems, info: {source, trigger}} = e.detail;
    items = newItems;
    // Ensure dragging is stopped on drag finish via keyboard
    if (source === SOURCES.KEYBOARD && trigger === TRIGGERS.DRAG_STOPPED) {
      dragDisabled = true;
    }
  }
  function handleFinalize(e) {
    const {items: newItems, info: {source}} = e.detail;
    items = newItems;
    // Ensure dragging is stopped on drag finish via pointer (mouse, touch)
    if (source === SOURCES.POINTER) {
      dragDisabled = true;
    }
  }

  let overResizeHandle = false
  function mouseMove(e) {
    const { x, y, target } = e
    const rect = target.getBoundingClientRect();
    // If the click is in the bottom right corner.
    overResizeHandle = rect.width - (x - rect.x) < 10 && rect.height - (y - rect.y) < 10
    dragDisabled = overResizeHandle
  }
  function startDrag(e) {
    // preventing default to prevent lag on touch devices (because of the browser checking for screen scrolling)

    if (overResizeHandle) {
      dragDisabled = true;
    } else {
      e.preventDefault();
      dragDisabled = false;
    }
  }
  function handleKeyDown(e) {
    if ((e.key === "Enter" || e.key === " ") && dragDisabled) dragDisabled = false;
  }

  const observer = new MutationObserver((mutationList) => {
    for (const { target, type } of mutationList) {
      if (!(target instanceof HTMLElement)) continue;

      if (type === 'attributes' && Boolean(target.style.width)) {
        const itemId = Array.from(element.childNodes).indexOf(target);
        const item = items[itemId];
        if (!item) continue;
        
        item.width = parseInt(target.style.width) / GRID_SPACING |0;
        target.style.width = '';
        console.log(item)
        
        items = items;
      }
    }
  })

</script>

<article
  bind:this={element}
  use:dndzone={{items, flipDurationMs, dragDisabled}}
  on:consider={handleConsider}
  on:finalize={handleFinalize}
>
  {#each items as item (item.id)}
    <section 
      animate:flip="{{ duration: flipDurationMs }}"
      style:grid-column="span {item.width}"
      aria-label="drag-handle"
      class="handle" 
      style={dragDisabled ? 'cursor: grab' : 'cursor: grabbing'}
      on:mousemove={mouseMove}
      on:mousedown={startDrag}
      on:touchstart={startDrag}
      on:keydown={handleKeyDown}
    >
      <slot item={item} />
    </section>
  {/each}
</article>

<style>
  article {
    display: grid;
    grid: 1fr / repeat(auto-fill, 1rem);
    gap: 0.2rem;
    padding: 0.2rem;

    border: thin solid black;

    height: 2rem
  }
  section {
    box-sizing: border-box;
    border: thin solid blue;
    text-align: center;
    line-height: 2rem;

    overflow: hidden;
    resize: horizontal;
  }
</style>