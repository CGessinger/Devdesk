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
		<TopBar {focus}/>
		{#if focus}
			<ContentPanel {focus}/>
		{/if}
		<button id="add_project" class="fa">&#xf067;</button>
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

    #add_project {
        display: inline-block;
		position: fixed;
        border: none;
        cursor: pointer;
		bottom: 0;
		right: 0;
		padding: 1rem;
		margin: 1rem;
		background-color: #912F40;
		color: whitesmoke;
    }
</style>
