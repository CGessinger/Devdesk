<script lang="ts">
    import { onMount } from "svelte";

    export let getScrollTarget: () => any;

    let progressContainer;
	let progressBar;
    let scrollTarget;

    onMount(() => {
        scrollTarget = getScrollTarget();
        scrollTarget.addEventListener("scroll", scrollEvent);
    });

	function scrollEvent(e) {
		const maxScroll = scrollTarget.scrollHeight - scrollTarget.getBoundingClientRect().height;
		const progress = scrollTarget.scrollTop / maxScroll * 0.75;
		progressBar.style.marginRight = progress * 100 + "%";
	}

	function clickScroll(e) {
		const targetRect = progressContainer.getBoundingClientRect();
		const y = e.clientY - targetRect.top;
		const progress = y / targetRect.height * 0.75;
		scrollTarget.scroll({
			top: progress * scrollTarget.scrollHeight,
  			behavior: 'smooth'
		});
	}

	let dragging = false;

	function dragStart(e) {
        // ToDo change cursor
		dragging = true;
	}

	function dragEnd(e) {
        // ToDo change cursor
		dragging = false;
	}

	function mouseMove(e) {
		if(!dragging)
			return;

		const target = progressContainer;
		const targetRect = target.getBoundingClientRect();
		const y = e.clientY - targetRect.top;
		const progress = y / targetRect.height * 0.75;
		scrollTarget.scroll({
			top: progress * scrollTarget.scrollHeight,
			behavior: 'instant'
		});
	}
</script>

<svelte:window on:mousemove={mouseMove} on:mouseup={dragEnd}/>

<div class="progress bg-dark" 
    bind:this="{progressContainer}" 
    on:mousedown|preventDefault="{clickScroll}">
    <div class="progress-bar h-100 w-25 ms-auto" 
        bind:this="{progressBar}" 
        on:mousedown={dragStart}>
    </div>
</div>

<style>
	.progress {
		transform: rotate(-90deg);
		transform-origin: right top;
		-webkit-transform: rotate(-90deg);
		-webkit-transform-origin:right top;
		width: calc(100vh - 4rem);
		margin-left: auto;
        grid-column: 1;
        grid-row: 1;
	}

	.progress-bar {
		background-color: rgba(255, 255, 255, 0.1);
        cursor: grab;
	}
</style>