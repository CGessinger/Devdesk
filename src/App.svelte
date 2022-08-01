<script lang="ts">
	import { open } from "@tauri-apps/api/dialog";
	import { documentDir } from "@tauri-apps/api/path";
	import { Portfolio } from "./Utils/Portfolio";
	import TopBar from "./Components/TopBar.svelte";

	import PortfolioItem from "./Components/PortfolioItem.svelte";

	let portfolios: Portfolio[] = [];

	async function add_portfolio(): Promise<void> {
		const selected = await open({
			directory: true,
			multiple: false,
			defaultPath: await documentDir(),
		});
		portfolios = [...portfolios, new Portfolio(selected.toString())];
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
					<PortfolioItem {portfolio}/>
				{/each}
			</ul>
			<span id="credentials">Made By <a href="https://www.github.com/CGessinger">CGessinger</a></span>
		</div>
	</div>
	<div id="panel_center">
		<TopBar/>
		<div id="portfolio_view" >
			<ul>

			</ul>
		</div>
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
		width: 20%;
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

	#panel_center {
		flex-grow: 5;
		height: 100%;
	}

	#portfolio_view {
		height: 100%;
	}

	#portfolio_view ul {
		list-style: none;
		padding: 0;
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
