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
		progressBar.style.marginLeft = progress * 100 + "%";
	}

	function clickScroll(e) {
		const targetRect = progressContainer.getBoundingClientRect();
		const y = e.clientY - targetRect.top;
		const progress = y / targetRect.height;
		scrollTarget.scroll({
			top: progress * (scrollTarget.scrollHeight - targetRect.height),
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
		const progress = y / targetRect.height;
		scrollTarget.scroll({
			top: progress * (scrollTarget.scrollHeight - targetRect.height),
			behavior: 'instant'
		});
	}
</script>

<svelte:window on:mousemove={mouseMove} on:mouseup={dragEnd}/>

<div class="progress bg-scheme" 
    bind:this={progressContainer}
    on:mousedown|preventDefault="{clickScroll}">
    <div class="progress-bar h-100 w-25" 
        bind:this="{progressBar}" 
        on:mousedown|preventDefault={dragStart}>
    </div>
</div>

<style>
	.progress {
		position: absolute;
		display: inline-block;
		transform: rotate(90deg);
		transform-origin: left top;
		-webkit-transform: rotate(90deg);
		-webkit-transform-origin: left top;
		width: calc(100vh - 1rem);
		top: 0;
		margin-top: 0.5rem;
		margin-left: calc(100% - 0.4rem);
	}

	.progress-bar {
		background-color: rgba(255, 255, 255, 0.2);
        cursor: grab;
		margin-right: auto;
	}
</style>