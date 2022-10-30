<script lang="ts">
    import ProjectTileView from "$src/Project/ProjectTileView.svelte";
	import { ProjectModel } from "$src/Project/utils/ProjectModel";
    import { ProjectModelBuilder } from "$src/Project/utils/ProjectModelBuilder";
	import { StateController } from "$src/store";
    import { listen } from "@tauri-apps/api/event";
	import ScrollBarComponent from "$src/Components/ScrollBarComponent.svelte";
    import { Portfolio } from "$utils/Data";

	export let data: Portfolio.Model;
	let projects: ProjectModel[] = [];
	let projectFiltered: ProjectModel[] | null = null;
	$: {
		Portfolio.getProjectsFromDatabase(data).then(pr => {
			projects = pr;
		})
	}
	
	listen<string>('searchInputChange', (event) => {
		if (event.payload == "") {
			projectFiltered = null;
		} else {
			projectFiltered = projects.filter(p => p.name.toLowerCase().includes(event.payload.toLowerCase()));
		}
	})

	async function addProject() {
		// ToDo use ProjectBuilder instead
		const p = new ProjectModel();
		p.type = Portfolio.subDirFilterString(data);
		const builder = new ProjectModelBuilder({
			targetPortfolioPath: data.path,
			type: Portfolio.subDirFilterString(data)
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
			{#each projectFiltered ?? projects as project}
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
		height: 100%;
		display: inline-block;
		width: 100%;
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
		display: flex;
		padding: 1rem 0 1rem 0;
		gap: 1rem;
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
