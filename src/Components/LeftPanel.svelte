<script lang="ts">
    import { Settings, Portfolio } from "$utils/Data";
    import { WindowStates, type StateHolder } from "$utils/ComponentStateController";
    import VaultTagList from "./VaultTagList.svelte";
    import SubdirList from "./SubdirList.svelte";
    import TextSearchComponent from "./TextSearchComponent.svelte";
    import HeaderComponent from "./HeaderComponent.svelte";
    import Credits from "./Credits.svelte";
    import NewProjectComponent from "./NewProjectComponent.svelte";

	export let state: StateHolder;


    let activePortfolio = null;
    $: {
        if(state.windowState == WindowStates.Portfolio) {
            activePortfolio = state.value;
        }
    }

	let portfolios: Portfolio.Model[] = [];
	Settings.get_portfolios().then(ps => { portfolios = ps; });

</script>

<div class="left-panel text-bg-scheme h-100">
	<HeaderComponent/>
	<div class="container selection-lists">
		<TextSearchComponent/>
		<hr>
		<VaultTagList {portfolios} {activePortfolio}/>
		<hr>
		<SubdirList {activePortfolio}/>
	</div>
	<NewProjectComponent/>
	<Credits/>
</div>

<style>
	.left-panel {
		width: 350px;
		display: flex;
		flex-direction: column;
		position: relative;
	}

	.left-panel > .selection-lists {
		flex-grow: 1;
	}
</style>
