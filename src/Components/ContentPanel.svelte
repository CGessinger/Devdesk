<script lang="ts">
	import type { Portfolio } from "../Utils/Portfolio";

	export let focus: Portfolio;
</script>

<div id="portfolio_view">
    <ul>
		{#each focus.projects as project}
			<li>{project}</li>
		{/each}
    </ul>
</div>

<style>
	#portfolio_view {
		height: 100%;
	}

	#portfolio_view ul {
		list-style: none;
		padding: 0;
		margin: 0;
	}
</style>