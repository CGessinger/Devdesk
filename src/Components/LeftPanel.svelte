<script lang="ts">
	import { StateController } from "$src/store";
	import { getVersion } from '@tauri-apps/api/app';
    import { Settings, Portfolio } from "$utils/Data";
    import { WindowStates, type StateHolder } from "$utils/ComponentStateController";
    import VaultTagList from "./VaultTagList.svelte";
    import SubdirList from "./SubdirList.svelte";

	export let state: StateHolder;

	let appVersion = "";
	getVersion().then((v) => {
		appVersion = v;
	});

    let activePortfolio = null;
    $: {
        if(state.windowState == WindowStates.Portfolio) {
            activePortfolio = state.value;
        }
    }

	let portfolios: Portfolio.Model[] = [];
	Settings.get_portfolios().then(ps => { portfolios = ps; });


	function focusSettings() {
		StateController.switchToSettings();
	}

</script>

<div class="left-panel container d-flex flex-column flex-shrink-0 p-0 text-bg-scheme h-100 mx-0">
	<div class="container h-100">
		<div class="d-flex align-items-center mb-3 pt-3 text-on-dark">
			<h2>Portfolios</h2>
			<i class="settings bi bi-gear mb-1 ms-3" on:click={_ => focusSettings()}/>
		</div>
		<hr>
		<VaultTagList {portfolios} {activePortfolio}/>
		<hr>
		<SubdirList {activePortfolio}/>
	</div>
	<!-- <button id="add_portfolio" class="fa" on:click={(_) => add_portfolio()}>&#xf067;</button> -->
	<div class="container color-accent">
		<span class="text-on-dark text-decoration-none container">
			Made By
			<a class="text-on-dark text-decoration-none fw-bold" href="https://www.github.com/CGessinger"> CGessinger</a>
			<span>({appVersion})</span>
		</span>
	</div>
</div>

<style>
	.left-panel {
		width: 350px;
	}

	.color-accent {
		background-color: #78002e;
	}
</style>
