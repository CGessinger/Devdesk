<script lang="ts">
	import {
		focused_portfolio,
		focused_project,
		focus_settings,
		cached_settings,
		new_project
	} from "$src/store";
	import { SettingsModel } from "$src/Settings/utils/SettingsModel";
	import TopBar from "./Components/TopBar.svelte";
	import LeftPanel from "./Components/LeftPanel.svelte";
	import PortfolioView from "./Portfolio/Display/PortfolioDisplayView.svelte";
	import ProjectView from "./Project/Display/ProjectDisplayView.svelte";
	import NewProjectView from "./Project/Create/ProjectCreateView.svelte";
	import SettingsView from "./Settings/Display/SettingsDisplayView.svelte";
    import type { ProjectModel } from "./Project/utils/ProjectModel";

	let settings: SettingsModel;
	cached_settings.subscribe((value) => (settings = value));
	SettingsModel.get_settings_from_config().then((s_) => {
		cached_settings.update(s => (s = s_))
	});

    $: {
        if (settings.dark_mode) {
            document.documentElement.setAttribute('data-theme', 'dark');
        }
        else {
            document.documentElement.setAttribute('data-theme', 'light');
        }    
    }

	let focus_project: ProjectModel;
	focused_project.subscribe((value) => {
		focus_project = value;
	});
</script>

<div id="beta_alert">This is a beta and is not fully tested yet. Use at your own risk!</div>
<div id="main">
	<div id="panel_left">
		<LeftPanel/>
	</div>
	<div id="panel_center">
		<TopBar/>
		{#if $focus_settings}
			<SettingsView/>
		{:else if $new_project}
			<NewProjectView edit={$new_project}/>
		{:else if focus_project}
			<ProjectView project={focus_project} />
		{:else if $focused_portfolio}
			<PortfolioView/>
		{/if}
	</div>
</div>

<style>
	#beta_alert {
		width: 100%;
		background-color: rgba(255, 0, 0, 0.6);
		z-index: 100;
		text-align: center;
		color: white;
		position: fixed;
		height: 0.8rem;
		font-size: 0.5rem;
	}

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
