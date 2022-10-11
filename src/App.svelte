<script lang="ts">
	import { cached_settings, StateController } from "$src/store";
	import { SettingsModel } from "$src/Settings/utils/SettingsModel";
	import TopBar from "./Components/TopBar.svelte";
	import LeftPanel from "./Components/LeftPanel.svelte";
    import { projectdb } from "./utils/Database";

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
<div id="main" class="d-flex flex-nowrap h-100 w-100">
	<LeftPanel/>
	<div class="w-100 h-100 container">
		<TopBar/>
		<svelte:component this="{component}" {data}></svelte:component>
	</div>
</div>

<style>
</style>
