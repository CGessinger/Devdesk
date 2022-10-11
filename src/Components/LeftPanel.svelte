<script lang="ts">
	import { cached_settings, StateController } from "$src/store";
	import { fs } from "$utils/Path"
	import { SettingsModel } from "$src/Settings/utils/SettingsModel";
	import { PortfolioModel } from "$src/Portfolio/utils/PortfolioModel";
	import { getVersion } from '@tauri-apps/api/app';

	let appVersion = "";
	getVersion().then((v) => {
		appVersion = v;
	});

	$: value = $StateController.value;
    $: prevValue = $StateController._prevValue;

    $: activePortfolio = ((): PortfolioModel => {
        if (value instanceof PortfolioModel) 
            return value;
        else if (prevValue instanceof PortfolioModel)
            return prevValue;

        return null;
    })()

	let s: SettingsModel;
	cached_settings.subscribe((value) => (s = value));

	function addPortfolio(_e) {
		s.addPortfolio().finally(() => {
			s.safeSettings();
			cached_settings.update((settings) => (settings = s));
		});
	}

	function removePortfolio(_e) {
		s.removePortfolioByPath(activePortfolio.path).finally(() => {
			StateController.switchToPortfolio(s.portfolios[s.portfolios.length - 1])
			s.safeSettings();
			cached_settings.update((settings) => (settings = s));
		});
	}

	function focusPortfolio(f: PortfolioModel) {
		StateController.switchToPortfolio(f);
	}

	function focusSettings() {
		if (value instanceof SettingsModel) {
			StateController.switchToPrev();
			return;
		}

		StateController.switchToSettings(s);
	}
</script>

<div class="container d-flex flex-column flex-shrink-0 p-3 text-bg-dark h-100 mx-0">
	<div class="d-flex align-items-center mb-3 mb-md-0 me-md-auto text-white">
		<h2>Portfolios</h2>
		<i class="settings bi bi-gear mb-1 ms-3" on:click={focusSettings}/>
	</div>
	<hr>
	<ul class="nav nav-pills flex-column">
		{#each s.portfolios as portfolio}
			<li class="nav-item">
				<span class="nav-link text-white px-1" class:active="{activePortfolio && activePortfolio.path == portfolio.path}" on:click={(_) => focusPortfolio(portfolio)}>
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
	<div class="text-white ms-me-auto text-center mb-auto mt-2">
		<button class="btn btn-dark" type="button" on:click="{addPortfolio}">
			<i class="bi bi-plus text-white"/>
		</button>
		<button class="btn btn-dark" type="button" on:click="{removePortfolio}">
			<i class="bi bi-trash"/>
		</button>
	</div>
	<!-- <button id="add_portfolio" class="fa" on:click={(_) => add_portfolio()}>&#xf067;</button> -->
	<span class="text-white text-decoration-none">
		Made By
		<a class="text-white text-decoration-none fw-bold" href="https://www.github.com/CGessinger"> CGessinger</a>
		<span>({appVersion})</span>
	</span>
</div>

<style>
	.container {
		width: 350px;
	}

	.nav-item,
	.settings {
		cursor: pointer;
	}

	.nav-link.active {
		background-color: rgba(255, 255, 255, 0.1);
	}
</style>
