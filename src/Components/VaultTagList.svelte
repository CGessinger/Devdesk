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
        class="text-on-dark btn-scheme" 
        class:active="{activePortfolio && activePortfolio.path == portfolio.path}" 
        on:click={(_) => focusPortfolio(portfolio)}>
            {fs.splitPath(portfolio.path).slice(-1).join(" / ")}
        </span>
    {/each}
    <div class="text-on-dark">
        <span class="add btn-scheme" on:click="{addPortfolio}">
            <i class="bi bi-plus"/>
        </span>
        <span class="remove btn-scheme" on:click="{removePortfolio}">
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
        padding: 0.2rem 0.6rem;
	}

	.vaults > span.active {
        background-color: #78002e;
	}

    .vaults span.add,
    .vaults span.remove {
        padding: 0.2rem 1rem !important;
    }

</style>