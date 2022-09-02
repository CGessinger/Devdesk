<script lang="ts">
	import {
		focused_portfolio,
		focused_project,
		focus_settings,
		cached_settings,
		new_project
	} from "$src/store";
	import { Settings } from "$utils/Settings";
	import type { Portfolio } from "$utils/Portfolio";
	import TopBar from "./Components/TopBar.svelte";
	import LeftPanel from "./Components/LeftPanel.svelte";
	import PortfolioView from "./Components/PortfolioView.svelte";
	import type { Project } from "$src/utils/Project";
	import ProjectView from "./Components/ProjectView.svelte";
	import NewProjectView from "./Components/NewProjectView.svelte";
	import SettingsView from "./Components/SettingsView.svelte";

	let s: Settings;
	cached_settings.subscribe((value) => (s = value));
	Settings.get_settings_from_config().then((s_) =>
		cached_settings.update((s) => (s = s_))
	);
    $: {
        if (s.dark_mode) {
            document.documentElement.setAttribute('data-theme', 'dark');
        }
        else {
            document.documentElement.setAttribute('data-theme', 'light');
        }    
    }

	let focus: Portfolio;
	focused_portfolio.subscribe((value) => {
		focus = value;
	});

	let focus_project: Project;
	focused_project.subscribe((value) => {
		focus_project = value;
	});
</script>

<div id="main">
	<div id="panel_left">
		<LeftPanel/>
	</div>
	<div id="panel_center">
		<TopBar/>
		{#if $focus_settings}
			<SettingsView/>
		{:else if $new_project}
			<NewProjectView/>
		{:else if focus_project}
			<ProjectView project={focus_project} />
		{:else if focus}
			<PortfolioView />
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
