<script lang='ts'>
  import { createEventDispatcher } from 'svelte';
  import { dndzone, SOURCES, TRIGGERS	 } from 'svelte-dnd-action';
  import TrackItem from './TrackItem.svelte';

  const dispatch = createEventDispatcher();
  const GRID_SPACING = 16;

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

    dispatch('change', items)
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
  const observer = new ResizeObserver((elements) => {
    for (const { target } of elements) {
      if (!(target instanceof HTMLElement)) continue;
      
      if (Boolean(target.style.width)) {
        const itemId = Array.from(element.children).indexOf(target);
        const item = items[itemId];
        console.log(itemId, item)
        if (!item) continue;
        
        let width = parseInt(target.style.width) / GRID_SPACING |0;
        if (width !== item.width) {
          item.width = width;
          dispatch('change', items)
          items = items;
        }
        target.style.width = '';
      }
    }
  })
</script>

<article
  bind:this={element}
  use:dndzone={{items, dragDisabled}}
  on:consider={handleConsider}
  on:finalize={handleFinalize}
>
  {#each items as item (item.id)}
    <TrackItem bind:width={item.width} {observer} {mouseMove} {startDrag} {handleKeyDown}>
      <slot item={item} />
    </TrackItem>
  {/each}
</article>

<style>
  article {
    display: grid;
    grid: 1fr / repeat(auto-fill, 1.5rem);
    gap: 0.2rem;
    padding: 0.2rem;

    border: thin solid black;

    height: 2rem
  }
</style>