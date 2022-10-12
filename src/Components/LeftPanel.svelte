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

<div class="left-panel container d-flex flex-column flex-shrink-0 p-0 text-bg-scheme h-100 mx-0">
	<div class="container h-100">
		<div class="d-flex align-items-center mb-3 pt-3 text-on-dark">
			<h2>Portfolios</h2>
			<i class="settings bi bi-gear mb-1 ms-3" on:click={focusSettings}/>
		</div>
		<hr>
		<ul class="nav nav-pills flex-column">
			{#each s.portfolios as portfolio}
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
