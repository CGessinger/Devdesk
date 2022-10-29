<script lang="ts">
	import { StateController } from "$src/store";
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
<div id="main">
	<div class="left-panel">
		<LeftPanel {state}/>
	</div>
	<div class="right-panel">
		<svelte:component this="{state.getComponent()}" data={state.value}></svelte:component>
	</div>
</div>

<style>
	#main {
		display: flex;
		flex-direction: row;
		height: 100vh;
		width: 100vw;
		gap: 1rem;
	}

	.left-panel {
		flex-basis: calc(100% / 5);
	}

	.right-panel {
		flex-grow: 1;
	}
</style>
