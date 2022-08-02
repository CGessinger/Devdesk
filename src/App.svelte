<script lang="ts">
	import { settings } from "./Utils/Settings";
	import TopBar from "./Components/TopBar.svelte";
	import LeftPanel from "./Components/LeftPanel.svelte";
	import ContentPanel from "./Components/ContentPanel.svelte";
	import type { Portfolio } from "./Utils/Portfolio";

	let s: settings = new settings();
	settings.get_settings_from_config().then(s_ => s = s_);

	let focus: Portfolio = undefined;
	let set_focus = (p: Portfolio) => {
		p.load_projects().then(() => focus = p);
	};
</script>

<main>
	<div id="panel_left">
		<LeftPanel {s} {set_focus}/>
	</div>
	<div id="panel_center">
		<TopBar/>
		{#if focus}
			<ContentPanel {focus}/>
		{/if}
	</div>
</main>

<style>
	main {
		margin: 0;
		padding: 0;
		width: 100%;
		height: 100%;

		display: flex;
		align-items: center;
		justify-content: center;
		column-gap: 1rem;
	}

	#panel_left {
		flex-grow: 1;
		height: 100%;
		width: 20%;
	}

	#panel_center {
		flex-grow: 5;
		height: 100%;
	}
</style>
