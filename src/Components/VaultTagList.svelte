<script lang="ts">
    import { StateController } from "$src/store";
    import { projectdb } from "$src/utils/Database";

    import { Portfolio, Settings } from "$utils/Data";
    import { fs } from "$utils/Path";

    export let activePortfolio: Portfolio.Model | null = null;
    export let portfolios: Portfolio.Model[] = [];
	
	function focusPortfolio(p: Portfolio.Model) {
        projectdb.clear_db();
        Portfolio.loadProjectsToDatabase(p);
		StateController.switchToPortfolio(p);
	}
    
	function addPortfolio(_e) {
		Settings.addPortfolioFromDialog().then(res => {
			portfolios = res;
		});
	}

	function removePortfolio(_e) {
		Settings.remove_portfolio(activePortfolio.path).then(res => {
			portfolios = res;
		});
	}
</script>

<div class="vaults">
    {#each portfolios as portfolio}
        <span 
        class="text-on-dark px-1" 
        class:active="{activePortfolio && activePortfolio.path == portfolio.path}" 
        on:click={(_) => focusPortfolio(portfolio)}>
            {fs.splitPath(portfolio.path).slice(-1).join(" / ")}
        </span>
    {/each}
    <div class="text-on-dark">
        <span class="add" on:click="{addPortfolio}">
            <i class="bi bi-plus"/>
        </span>
        <span class="remove" on:click="{removePortfolio}">
            <i class="bi bi-trash"/>
        </span>
    </div>
</div>

<style>
    .vaults {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: flex-start;
        align-items: center;
        align-content: flex-start;
        gap: 0.5rem;
    }

	.vaults span {
		cursor: pointer;
        font-size: 0.9rem;
        text-transform: capitalize;
        border-radius: 2em;
        display: inline-block;
        padding: 0.2em 0.5em;
        border: 2px solid rgba(255, 255, 255, 0.1);
	}

	.vaults > span.active {
		background-color: rgba(255, 255, 255, 0.1);
	}

    .vaults span:hover {
        background-color: rgba(255, 255, 255, 0.1);
    }

</style>