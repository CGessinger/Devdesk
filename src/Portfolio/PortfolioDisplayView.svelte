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

<div class="portfolio-scroll">
	<ul class="overflow-scroll h-100 list-group">
		{#each projects as project}
		<li on:click={(_) => project_click(project)}>
			<ProjectTileView {project}/>
		</li>
		{/each}
	</ul>
	<button class="add-project btn btn-dark sticky-bottom border border-white" on:click={(_) => add_project()}>
		<i class="bi bi-plus"/>
	</button>
</div>

<style>
	.portfolio-scroll {
		height: calc(100% - 3rem - 1rem);		
	}

	.add-project {
		width: 4rem;
		height: 4rem;
		float: right;
		bottom: 1rem;
	}
</style>
