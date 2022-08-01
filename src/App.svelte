<script lang="ts">
	import { open } from "@tauri-apps/api/dialog";
	import { documentDir } from "@tauri-apps/api/path";

	import PortfolioItem from "./Components/PortfolioItem.svelte";

	let portfolios = [];
	let selected_portfolio: number = 0;

	async function add_portfolio(): Promise<void> {
		const selected = await open({
			directory: true,
			multiple: false,
			defaultPath: await documentDir(),
		});
		portfolios = [...portfolios, selected];
	}
</script>

<main>
	<div id="panel_left">
		<div id="portfolio_list_view">
			<h2>
				Portfolios
				<span id="add_portfolio" on:click={add_portfolio}>+</span>
			</h2>
			<ul id="portfolio_list">
				{#each portfolios as portfolio, i}
					<li>
						<PortfolioItem {portfolio}/>
					</li>
				{/each}
			</ul>
		</div>
	</div>
	<div id="panel_center">
		<div id="portfolio_view" />
	</div>
</main>

<style>
	main {
		margin: 0;
		padding: 0;
		width: 100%;
		height: 100%;

		display: flex;
		align-items: center;
		justify-content: center;
		column-gap: 1rem;
	}

	#panel_left {
		flex-grow: 1;
		height: 100%;
	}

	#portfolio_list_view {
		height: 100%;
		background-color: gray;
		border-radius: 20px;
	}

	#portfolio_list_view h2 {
		margin: 0;
		padding: 0;
		text-align: center;
		font-size: 1.5rem;
		color: black;
	}

	#add_portfolio {
		cursor: pointer;
	}

	#portfolio_list {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	#portfolio_list li {
		padding: 0.5rem;
		margin: 1rem;
		cursor: pointer;
		background-color: darkgray;
		border-radius: 20px;
	}

	#panel_center {
		flex-grow: 2;
		height: 100%;
	}

	#portfolio_view {
		height: 100%;
		background-color: gray;
		border-radius: 20px;
	}
</style>
