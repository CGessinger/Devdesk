<script lang="ts">
	import {
		focused_portfolio,
		focused_project,
		cached_settings,
		focus_settings,
		new_project
	} from "$src/store";
	import { fs } from "$utils/Path"
	import type { SettingsModel } from "$src/Settings/utils/SettingsModel";
	import type { PortfolioModel } from "$src/Portfolio/utils/PortfolioModel";
	import { getVersion } from '@tauri-apps/api/app';
    import PortfolioDisplayView from "$src/Portfolio/PortfolioDisplayView.svelte";
    import { path } from "@tauri-apps/api";

	let appVersion = "";
	getVersion().then((v) => {
		appVersion = v;
	});

	let activePath = "";
	focused_portfolio.subscribe((fp) => {
		if (fp)
			activePath = fp.path
	});

	let s: SettingsModel;
	cached_settings.subscribe((value) => (s = value));

	function addPortfolio(_e) {
		s.addPortfolio().finally(() => {
			s.safeSettings();
			cached_settings.update((settings) => (settings = s));
		});
	}

	function removePortfolio(_e) {
		s.removePortfolioByPath(activePath).finally(() => {
			focused_portfolio.set(s.portfolios[s.portfolios.length - 1])
			s.safeSettings();
			cached_settings.update((settings) => (settings = s));
		});
	}

	function set_focus(f: PortfolioModel) {
		focused_project.update((pr) => (pr = undefined));
		focus_settings.update((fs) => (fs = false));
        new_project.update((np) => (np = undefined));
		focused_portfolio.update((p) => (p = f));
	}

	function set_focus_settings() {
		focus_settings.update((fs) => (fs = !fs));
	}
</script>

<div class="container d-flex flex-column flex-shrink-0 p-3 text-bg-dark h-100">
	<div class="d-flex align-items-center mb-3 mb-md-0 me-md-auto text-white">
		<h2>Portfolios</h2>
		<i class="settings bi bi-gear mb-1 ms-3" on:click={set_focus_settings}/>
	</div>
	<hr>
	<ul class="nav nav-pills flex-column">
		{#each s.portfolios as portfolio}
			<li class="nav-item">
				<span class="nav-link text-white px-1" class:active="{activePath == portfolio.path}" on:click={(_) => set_focus(portfolio)}>
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
	<span class="d-flex align-items-center text-white text-decoration-none">
		Made By
		<a class="text-white text-decoration-none" href="https://www.github.com/CGessinger"> CGessinger</a>
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
		background-color: var(--primary-color);
	}
</style>
