
<script lang='ts'>
  import type Color from 'color'
  import { createEventDispatcher } from 'svelte';

  export let color: Color
  const dispatch = createEventDispatcher()

	let el: HTMLElement;
	
	$: hue = color.hue()
	
	let dragstart =false


	const setHue = (x, y) => {
		if (!el) return

		const { width, height } = el.getBoundingClientRect()
    const hue = 180 - (Math.atan2(x / width - 0.5, y / height - 0.5) / Math.PI) * 180	
		color = color.hue(hue)

    dispatch('change', { value: color.rgb().array().map(v => v|0) })
	}
	const onMouse = (e) => {
		if (!dragstart) return 
		
		setHue(e.offsetX, e.offsetY)
	}
	const onTouch = (e) => {
		if (!dragstart || !el) return
		
		const { top, left } = el.getBoundingClientRect()
    const touch = e.touches[0];
    setHue(touch.clientX - left, touch.clientY - top)
	}
</script>

<article
				 bind:this={el}
				 style:--hue="{hue}deg"
				 on:mousedown={(event) =>{dragstart = true; onMouse(event)}}
				 on:mousemove={onMouse}
				 on:mouseup={() => {dragstart = false}}
				on:touchstart={(event) => {dragstart = true; onTouch(event)}}
	      on:touchmove={onTouch}
	      on:touchend={() => { dragstart = false}}
	>
	<div class='gradient' />
	<div class='pointer' />
</article>


<style>
	
	article {
		max-width: 10rem;
		position: relative;
		width: 100%;
		aspect-ratio: 1;
		
		--pointer-size: 60%;
		--hue: 120deg;
	}
	article > * {
		pointer-events: none;
	}
	
	.pointer {
		background: hsl(var(--hue), 100%, 50%);
		width: var(--pointer-size);
		height: var(--pointer-size);
		
		transform: rotate(calc(45deg + var(--hue)));
		
		position: absolute;
		left: 50%; top: 50%;
		margin-left: calc(var(--pointer-size) / -2);
		margin-top: calc(var(--pointer-size) / -2);
		
		border-radius: 50%;
		border-top-left-radius: 0;
		
		border: 1px solid #061612;
		box-shadow: 0px 0px 10px #061612;
		
		
		box-sizing: border-box;
	}

.gradient {
	position: absolute;
	top: 0; left: 0; right: 0; bottom: 0;
  border-radius: 50%;
  background: conic-gradient( 
		hsl(0, 100%, 50%) 0%,
		rgba(255, 154, 0, 1) 10%,
		rgba(208, 222, 33, 1) 20%,
		rgba(79, 220, 74, 1) 30%,
		rgba(63, 218, 216, 1) 40%,
		rgba(47, 201, 226, 1) 50%,
		rgba(28, 127, 238, 1) 60%,
		rgba(95, 21, 242, 1) 70%,
		rgba(186, 12, 248, 1) 80%,
		rgba(251, 7, 217, 1) 90%,
		rgba(255, 0, 0, 1) 100%);
}

</style>