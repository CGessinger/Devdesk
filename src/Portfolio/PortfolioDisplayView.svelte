<script lang="ts">
    import ProjectTileView from "$src/Project/ProjectTileView.svelte";
	import { ProjectModel } from "$src/Project/utils/ProjectModel";
    import { ProjectModelBuilder } from "$src/Project/utils/ProjectModelBuilder";
	import { StateController } from "$src/store";
    import { projectdb } from "$src/utils/Database";
    import { listen } from "@tauri-apps/api/event";
    import type { PortfolioModel } from "./utils/PortfolioModel";

	export let data: PortfolioModel;

	let projects: ProjectModel[] = [];
	$: (async () => {
		projects = await data.getProjects();
	})();

	listen<string>('searchInputChange', (event) => {
		const query = new projectdb.query({ textSearch: [event.payload] });
		(async () => {
			projects = await data.getProjects(query);
		})();
	})

	async function addProject() {
		// ToDo use ProjectBuilder instead
		const p = new ProjectModel();
		p.type = data.getFocusedTypeString();
		StateController.switchToProjectCreation(new ProjectModelBuilder(p).withPortfolio(data));
	}

	function clickProject(p: ProjectModel) {
		const project = new ProjectModel();
		Object.assign(project, p);
		StateController.switchToProject(project);
	}
</script>

<div class="portfolio-scroll">
	<ul class="overflow-scroll h-100 list-group">
		{#each projects as project}
		<li on:click={(_) => clickProject(project)}>
			<ProjectTileView {project}/>
		</li>
		{/each}
	</ul>
	<button class="add-project btn btn-dark sticky-bottom border border-white" on:click={(_) => addProject()}>
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
