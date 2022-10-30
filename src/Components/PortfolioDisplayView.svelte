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

	function clickProject(p: ProjectModel) {
		const project = new ProjectModel();
		Object.assign(project, p);
		StateController.switchToProject(project);
	}

	let scrollTarget;
</script>

<div class="portfolio-display">
	<div class="portfolio-scroll">
		<ul class="h-100 list-group" bind:this="{scrollTarget}">
			{#each projectFiltered || projects as project}
			<li on:click={(_) => clickProject(project)}>
				<ProjectTileView {project}/>
			</li>
			{/each}
		</ul>
	</div>
	<ScrollBarComponent getScrollTarget={() => scrollTarget}/>
</div>

<style>
	.portfolio-display {
		position: relative;
		display: inline-block;
		height: 100%;
		width: 100%;
		padding: 0.5rem 0 0.5rem 0;
	}

	.portfolio-scroll {
		height: 100%;
		width: calc(100% - 1.5rem);
	}

	.list-group {
		-ms-overflow-style: none; /* for Internet Explorer, Edge */
		scrollbar-width: none; /* for Firefox */
		overflow-y: scroll;
		display: flex;
		gap: 1rem;
	}

	.list-group::-webkit-scrollbar {
		display: none; /* for Chrome, Safari, and Opera */
	}
</style>
