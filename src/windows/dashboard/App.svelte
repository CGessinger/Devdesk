<script lang="ts">
	import { settings } from "../../Utils/Settings";
	import TopBar from "../../windows/dashboard/Components/TopBar.svelte";
	import LeftPanel from "../../windows/dashboard/Components/LeftPanel.svelte";
	import ContentPanel from "../../windows/dashboard/Components/ContentPanel.svelte";
	import type { Portfolio } from "../../Utils/Portfolio";

	let s: settings = new settings();
	settings.get_settings_from_config().then(s_ => s = s_);

	let focus: Portfolio = undefined;
	let set_focus = (p: Portfolio) => {
		if (focus == p)
			return;

		if (p)
			p.load_projects_from_type().then(() => focus = p);
		else
			focus = undefined;
	};

</script>

<div id="main">
	<div id="panel_left">
		<LeftPanel {s} {set_focus}/>
	</div>
	<div id="panel_center">
		<TopBar {focus} on:safe-settings="{_ => s.safe_settings()}" on:refresh-focus="{_ => focus = focus}"/>
		{#if focus}
			<ContentPanel {focus}/>
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
</style>
