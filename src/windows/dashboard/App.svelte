<script lang="ts">
	import { settings } from "$utils/Settings";
	import type { Portfolio } from "$utils/Portfolio";
	import TopBar from "./Components/TopBar.svelte";
	import LeftPanel from "./Components/LeftPanel.svelte";
	import ContentPanel from "./Components/ContentPanel.svelte";
import { isErr } from "$src/utils/Result";

	let s: settings = new settings();
	settings.get_settings_from_config().then(s_ => s = s_);

	let focus: Portfolio = undefined;
	let set_focus = (p: Portfolio) => {
		if (focus == p)
			return;

		if (p) {
			p.load_projects_from_type().then(() => focus = p);
		}
		else
			focus = undefined;
	};

	function refresh_focus() {
		focus.projects.forEach(project => {
			project.load_image().then(res => {
				if (!isErr(res)) {
					focus = focus;
				}
			});
		});
		focus = focus;
	}

</script>

<div id="main">
	<div id="panel_left">
		<LeftPanel {s} {set_focus}/>
	</div>
	<div id="panel_center">
		<TopBar {focus} on:safe-settings="{_ => s.safe_settings()}" on:refresh-focus="{_ => refresh_focus()}"/>
		{#if focus}
			<ContentPanel {focus} on:refresh-focus="{_ => focus = focus}"/>
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
		display: inline-block;
		flex: 3;
	}
</style>
