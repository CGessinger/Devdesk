<script lang="ts">
	import { focused_portfolio, focused_project, cached_settings, focus_settings } from "$src/windows/store";
    import type { Settings } from "$utils/Settings";
	import type { Portfolio } from "$utils/Portfolio";

    let s: Settings;
	cached_settings.subscribe((value) => (s = value));

    function add_portfolio() {
		s.add_portfolio()
		.finally(() => {
			s.safe_settings();
			cached_settings.update(settings => settings = s);
		});
	}

	function set_focus(f: Portfolio) {
		focused_project.update(pr => pr = undefined);
		focus_settings.update(fs => fs = false);
		focused_portfolio.update(p => p = f);
	}

	function set_focus_settings() {
		focus_settings.update(fs => fs = !fs);
	}

</script>

<div id="portfolio_list_view">
    <h2> 
		Portfolios 
		<span id="settings" class="fa fa-cog" on:click="{set_focus_settings}"></span> 
	</h2>
    <ul id="portfolio_list">
        {#each s.portfolios as portfolio}
			<li class="portfolio_item">
				<h3 class="portfolio_head" on:click={_ => set_focus(portfolio)}>
					📖{portfolio.path}
				</h3>
			</li>
        {/each}
    </ul>

	<button id="add_portfolio" class="fa" on:click="{_ => add_portfolio()}">&#xf067;</button>
    <span id="credentials">Made By <a href="https://www.github.com/CGessinger">CGessinger</a></span>
</div>

<style>
	#portfolio_list_view {
		position: relative;
		display: inline-block;
		height: 100%;
		background-color: var(--primary-color);
		color: whitesmoke;
	}

    .portfolio_item {
		word-wrap: break-word;
        padding-top: 1rem;
        display: inline-block;
        width: 100%;
    }

    .portfolio_head {
        margin: 0;
        padding: 0;
        font-size: 1rem;
        cursor: pointer;
    }

	#portfolio_list_view h2 {
		margin: 0;
		padding: 1.5rem 0 0 0;
		text-align: center;
		font-size: 1.2rem;
	}

	#settings {
		padding-left: 1rem;
		font-size: 1.2rem;
		cursor: pointer;
	}

	#add_portfolio {
		position: absolute;
        display: inline-block;
        border: none;
        cursor: pointer;
		bottom: 0;
		right: 0;
		padding: 1rem;
		margin: 1rem;
		background-color: whitesmoke;
		color: #40434e;
	}

	#portfolio_list {
		list-style: none;
		padding: 1rem;
		margin: 0;
	}

	#credentials {
		position: fixed;
		bottom: 0;
		padding: 0 0 0.5rem 0.5rem;
	}
	
	#credentials a {
		color: whitesmoke;
		font-weight: bold;
	}
</style>