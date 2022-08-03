<script lang="ts">
    import type { settings } from "../Utils/Settings";
	import type { Portfolio } from "../Utils/Portfolio";

    export let s: settings;
	export let set_focus: (p: Portfolio) => void;

    function add_portfolio() {
		s.add_portfolio()
		.finally(() => {
			s.safe_settings();
			s = s;
		});
	}
</script>

<div id="portfolio_list_view">
    <h2 on:click="{_ => set_focus(undefined)}"> Portfolios </h2>
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
    .portfolio_item {
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

	#portfolio_list_view {
		position: relative;
		height: 100%;
		background-color: #912F40;
		color: whitesmoke;
	}

	#portfolio_list_view h2 {
		margin: 0;
		padding: 1.5rem 0 0 0;
		text-align: center;
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