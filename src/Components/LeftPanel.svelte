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
    <h2>
        Portfolios
        <span id="add_portfolio" on:click="{_ => add_portfolio()}">+</span>
    </h2>
    <ul id="portfolio_list">
        {#each s.portfolios as portfolio}
			<li class="portfolio_item">
				<h3 class="portfolio_head" on:click={() => (set_focus(portfolio))}>
					📖{portfolio.path}
				</h3>
			</li>
        {/each}
    </ul>
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
		height: 100%;
		background-color: #912F40;
		color: whitesmoke;
	}

	#portfolio_list_view h2 {
		margin: 0;
		padding: 1.5rem 0 0 0;
		text-align: center;
		font-size: 1.2rem;
	}

	#add_portfolio {
		cursor: pointer;
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