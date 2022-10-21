<script lang="ts">
	import { StateController } from "$src/store";
	import TopBar from "./Components/TopBar.svelte";
	import LeftPanel from "./Components/LeftPanel.svelte";
    import { Settings } from "$utils/Data";
    import type { StateHolder } from "$utils/ComponentStateController";

	let state: StateHolder = null;
	StateController.subscribe((value) => {
		state = value;
	});

	Settings.getSwitches().then((s) => {
        if (s.darkMode) {
            document.documentElement.setAttribute('data-theme', 'dark');
        }
	});
</script>

<!-- <div id="beta_alert">This is a beta and is not fully tested yet. Use at your own risk!</div> -->
<div id="main" class="d-flex flex-nowrap h-100 w-100">
	<LeftPanel {state}/>
	<div class="w-100 h-100 container">
		<TopBar {state}/>
		<svelte:component this="{state.getComponent()}" data={state.value}></svelte:component>
	</div>
</div>

<style>
</style>
