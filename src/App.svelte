<script lang="ts">
	import { cached_settings, StateController } from "$src/store";
	import { SettingsModel } from "$src/Settings/utils/SettingsModel";
	import TopBar from "./Components/TopBar.svelte";
	import LeftPanel from "./Components/LeftPanel.svelte";

	let settings: SettingsModel;
	cached_settings.subscribe((value) => (settings = value));
	SettingsModel.get_settings_from_config().then((s_) => {
		cached_settings.update(s => (s = s_));
	});

    $: {
        if (settings.dark_mode) {
            document.documentElement.setAttribute('data-theme', 'dark');
        }
        else {
            document.documentElement.setAttribute('data-theme', 'light');
        }    
    }

	let component = null;
	let data = null;
	StateController.subscribe((value) => {
		component = value.getComponent();
		data = value.value;
	});
</script>

<!-- <div id="beta_alert">This is a beta and is not fully tested yet. Use at your own risk!</div> -->
<div id="main" class="d-flex flex-nowrap h-100">
	<LeftPanel/>
	<div class="w-100 h-100 container">
		<TopBar/>
		<svelte:component this="{component}" {data}></svelte:component>
		<!-- {#if $focus_settings}
			<SettingsView/>
		{:else if $new_project}
			<NewProjectView edit={$new_project}/>
		{:else if focus_project}
			<ProjectView project={focus_project} />
		{:else if $focused_portfolio}
			<PortfolioView/>
		{/if} -->
	</div>
</div>

<style>
</style>
