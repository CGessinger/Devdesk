<script lang="ts">
    import ProjectTileView from "$src/Project/ProjectTileView.svelte";
	import { ProjectModel } from "$src/Project/utils/ProjectModel";
	import { focused_project, new_project, focused_portfolio } from "$src/store";
    import type { PortfolioModel } from "./utils/PortfolioModel";

	let projects = [];
	let portfolio: PortfolioModel;
	focused_portfolio.subscribe((fp) => {
		portfolio = fp;
		(async () => {
			projects = await portfolio.load_projects_from_type();
		})();
	})

	async function add_project() {
		// ToDo use ProjectBuilder instead
		var p = new ProjectModel();
		p.type = portfolio.get_focused_type();
		new_project.update((project) => (project = p));
	}

	function project_click(pr_: ProjectModel) {
		focused_project.update((pr) => (pr = pr_));
	}
</script>

<div id="portfolio_view">
	<ul>
		{#each projects as project}
		<span on:click={(_) => project_click(project)}>
			<ProjectTileView {project}/>
		</span>
		{/each}
	</ul>
	<button id="add_project" class="fa" on:click={(_) => add_project()}
		>&#xf067;</button
	>
</div>

<style>
	#portfolio_view {
		grid-row: 2;
		padding: 0 1rem;
		overflow-y: scroll;
	}

	#portfolio_view ul {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	#add_project {
		display: inline-block;
		position: fixed;
		border: none;
		cursor: pointer;
		bottom: 0;
		right: 0;
		padding: 1rem;
		margin: 1rem;
		background-color: var(--primary-color);
		color: whitesmoke;
		border: whitesmoke 4px solid;
	}
</style>
