<script lang="ts">
	import { focused_portfolio } from "$src/windows/store.js";
	import { settings } from "$utils/Settings";
	import type { Portfolio } from "$utils/Portfolio";
	import TopBar from "./Components/TopBar.svelte";
	import LeftPanel from "./Components/LeftPanel.svelte";
	import ContentPanel from "./Components/ContentPanel.svelte";

	let s: settings = new settings();
	settings.get_settings_from_config().then(s_ => s = s_);
	
	let focus: Portfolio;
	focused_portfolio.subscribe(value => {
		focus = value;
	});
</script>

<div id="main">
	<div id="panel_left">
		<LeftPanel {s}/>
	</div>
	<div id="panel_center">
		<TopBar on:safe-settings="{_ => s.safe_settings()}"/>
		{#if focus}
			<ContentPanel/>
		{/if}
	</div>
</div>

<style>
	#main {
		margin: 0;
		padding: 0;
		width: 100%;
		height: 100%;

		display: flex;
		align-items: stretch;
		justify-content: start;
	}

	#panel_left {
		display: inline-block;
		flex: 1;
	}

	#panel_center {
		display: grid;
		grid-template-rows: 5rem 1fr;
		flex: 3;
	}

</style>
