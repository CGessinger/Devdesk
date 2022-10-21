<script lang="ts">
	import { StateController } from "$src/store";
	import { fs } from "$utils/Path"
	import { getVersion } from '@tauri-apps/api/app';
    import { Settings, Portfolio } from "$utils/Data";
    import { WindowStates, type StateHolder } from "$utils/ComponentStateController";

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
	
	function focusPortfolio(p: Portfolio.Model) {
		StateController.switchToPortfolio(p);
	}

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
		<ul class="nav nav-pills flex-column">
			{#each portfolios as portfolio}
				<li class="nav-item">
					<span class="nav-link text-on-dark px-1" class:active="{activePortfolio && activePortfolio.path == portfolio.path}" on:click={(_) => focusPortfolio(portfolio)}>
						<!-- <i class="bi bi-book me-1"/> -->
						<!-- <span class="text-break">{portfolio.path}</span> -->
						<ol class="breadcrumb mb-0">
							{#each fs.splitPath(portfolio.path) as part}
								<li class="breadcrumb-item disabled">{part}</li>
							{/each}
						</ol>
					</span>
				</li>
			{/each}
		</ul>
		<div class="text-on-dark ms-me-auto text-center mb-auto mt-2">
			<button class="btn btn-scheme-darker" type="button" on:click="{addPortfolio}">
				<i class="bi bi-plus"/>
			</button>
			<button class="btn btn-scheme-darker" type="button" on:click="{removePortfolio}">
				<i class="bi bi-trash"/>
			</button>
		</div>
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

	.nav-item,
	.settings {
		cursor: pointer;
	}

	.nav-link.active {
		background-color: rgba(255, 255, 255, 0.1);
	}
</style>
