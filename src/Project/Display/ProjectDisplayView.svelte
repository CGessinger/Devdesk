<script lang="ts">
    import type { ProjectModel } from "../utils/ProjectModel";
    import { Model } from "./ProjectDisplayModel";

    export let project: ProjectModel;
    let model = new Model(project);
	let ViewData = model.GetViewData();
	model.onViewDataChange = (_) => {
		ViewData = model.GetViewData();
	}
</script>

<div id="project_view">
    <h1 id="project_name">
        {project.name}
        {#if ViewData["config_exists"]}
            <span id="edit_button" on:click="{_ => model.edit_model(project)}">Edit</span>
        {:else}
            <span id="edit_button" on:click="{_ => model.init_model(project)}">Init</span>
        {/if}
    </h1>
    <div id="scripts_nav">
        <span class="script_item" on:click="{_ => model.terminal_here()}">
            Terminal Here
        </span>
        <span class="script_item" on:click="{_ => model.vscode_here()}">
            VS Code Here
        </span>
    </div>
    <div id="project_description">
        {project.description}
    </div>
</div>

<style>
    #project_view {
		padding: 0 1rem;
        overflow-y: scroll;
    }

    #project_name {
        font-size: 2rem;
        font-weight: 600;
        margin: 0;
        padding: 0.5rem;
        text-align: center;
        color: var(--font-color-dark);
        background-color: var(--primary-color);
    }

    #edit_button {
        float: right;
        font-size: small;
        margin: 0.6rem;
    }

    #scripts_nav {
        margin: 0;
        padding: 0;
        margin-top: 0.8rem;
    }

    .script_item {
        display: inline-block;
        text-align: center;
        padding: 8px 10px;
        cursor: pointer;
        background-color: var(--primary-color);
        color: var(--font-color-dark);
        border-radius: 5px;
        cursor: pointer;
    }

    #project_description {
        font-size: 1.2rem;
        margin: 1rem 1rem 0 1rem;
        color: var(--font-color-dark);
    }
</style>