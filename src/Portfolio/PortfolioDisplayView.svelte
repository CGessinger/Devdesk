<script lang="ts">
    import ProjectTileView from "$src/Project/ProjectTileView.svelte";
	import { ProjectModel } from "$src/Project/utils/ProjectModel";
    import { ProjectModelBuilder } from "$src/Project/utils/ProjectModelBuilder";
	import { StateController } from "$src/store";
    import { projectdb } from "$utils/Database";
    import { listen } from "@tauri-apps/api/event";
	import ScrollBarComponent from "$src/Components/ScrollBarComponent.svelte";
    import { Portfolio } from "$utils/Data";

	export let data: Portfolio.Model;
	let projects: ProjectModel[] = [];
	$: {
		Portfolio.getProjectsFromDatabase(data).then(pr => {
			projects = pr;
		})
	}

	listen<string>('searchInputChange', (event) => {
		const query = new projectdb.query({ textSearch: [event.payload] });
		Portfolio.getProjectsFromDatabase(data, query).then(pr => {
			projects = pr;
		});
	})

	async function addProject() {
		// ToDo use ProjectBuilder instead
		const p = new ProjectModel();
		p.type = Portfolio.focusedTypeString(data);
		const builder = new ProjectModelBuilder({
			targetPortfolioPath: data.path,
			type: Portfolio.focusedTypeString(data)
		})
		StateController.switchToProjectCreation(builder);
	}

	function clickProject(p: ProjectModel) {
		const project = new ProjectModel();
		Object.assign(project, p);
		StateController.switchToProject(project);
	}

	let scrollTarget;
</script>

<div class="portfolio-scroll">
	<div class="grid h-100 pe-3">
		<ScrollBarComponent getScrollTarget={() => scrollTarget}/>
		<ul class="overflow-auto h-100 list-group me-1" bind:this="{scrollTarget}">
			{#each projects as project}
			<li on:click={(_) => clickProject(project)}>
				<ProjectTileView {project}/>
			</li>
			{/each}
		</ul>
	</div>
	<button class="add-project btn btn-scheme sticky-bottom border border-white me-1" on:click={(_) => addProject()}>
		<i class="bi bi-plus"/>
	</button>
</div>

<style>
	.portfolio-scroll {
		height: calc(100% - 3rem - 1rem);
	}

	.grid {
		display: grid;
	}

	.grid * {
        grid-column: 1;
        grid-row: 1;
	}

	.list-group {
		-ms-overflow-style: none; /* for Internet Explorer, Edge */
		scrollbar-width: none; /* for Firefox */
		overflow-y: scroll; 
	}

	.list-group::-webkit-scrollbar {
		display: none; /* for Chrome, Safari, and Opera */
	}

	.add-project {
		width: 4rem;
		height: 4rem;
		float: right;
		bottom: 1rem;
	}

	i.bi-plus {
		font-size: 1.5rem;
	}
</style>
