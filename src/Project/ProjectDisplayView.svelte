<script lang="ts">
    import type { ProjectModel } from "./utils/ProjectModel";
    import { terminal } from "$utils/Scripts";

    export let data: ProjectModel;
    let configExists = false;
    data.config_exists().then(exists => configExists = exists);

    function editModel() {
        // ToDo implement
        throw new Error("Not implemented"); 
        // StateController.switchToProjectCreation(new ProjectModelBuilder(data))
    }

    function initModel() {
        // ToDo implement
        throw new Error("Not implemented");
        // StateController.switchToProjectCreation(new ProjectModelBuilder(data))
    }

    function terminalHere() {
        terminal.terminal_here(data.path);
    }

    function vscodeHere() {
        terminal.vscode_here(data.path);
    }
</script>

<div id="project_view">
    <h1 id="project_name">
        {data.name}
        {#if configExists}
            <span id="edit_button" on:click="{_ => editModel()}">Edit</span>
        {:else}
            <span id="edit_button" on:click="{_ => initModel()}">Init</span>
        {/if}
    </h1>
    <div id="scripts_nav">
        <span class="script_item" on:click="{_ => terminalHere()}">
            Terminal Here
        </span>
        <span class="script_item" on:click="{_ => vscodeHere()}">
            VS Code Here
        </span>
    </div>
    <div id="project_description">
        {data.description}
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