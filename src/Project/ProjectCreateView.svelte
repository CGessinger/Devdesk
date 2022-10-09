<script lang="ts">
    import type { ProjectModelBuilder } from "./utils/ProjectModelBuilder";
    import type { ProjectModel } from "./utils/ProjectModel";
    import { ProjectFileHandler } from "./utils/ProjectFileHandler";
    import { StateController } from "$src/store";

    export let data: ProjectModelBuilder;

    let targetPathPreview: string = data.targetPath().value;
    let configExists = false;
    data.p.config_exists().then(exists => configExists = exists);

    function inputEvent(e, handler) {
        const val = e.target.value;
        handler(val);
    }
    
    function changeType(type: string) {
        data.withType(type);
        targetPathPreview = data.targetPath().value;
    }

    function changeName (name: string) {
        data.withName(name);
        targetPathPreview = data.targetPath().value;
    }

    function changeGitUrl(url: string) {
        data.withGitUrl(url);
    }

    function changeGitBranch(branch: string) {
        data.withGitBranch(branch);
    }

    function changeDescription(description: string) {
        data.withDescription(description);
    }

    async function createProject() {
        const res = data.tryBuildPath();
        if (res.is_err()) {
            console.log("error: ", res);
            return;
        }
        const built = data.build();
        if (built.is_err()) {
            console.log("error: ", built);
            return;
        }
        const fb = new ProjectFileHandler(built.unwrap() as ProjectModel);
        await fb.createConfigFolder();
        await fb.writeToConfig();
        fb.cloneGit();
        
        StateController.switchToPortfolio(data.targetPortfolio);
    }

    function editProject() {

    }
</script>

<div id="newproject_view">
    <div id="top_wrapper">
        <input type="text" id="project_name" placeholder="Project Name" on:input="{(e) => inputEvent(e, changeName)}" value="{data.p.name}"/>
    </div>
    <div id="main_wrapper">
        <div id="git">
            <input type="text" id="git_url" placeholder="Git URL" on:change="{(e) => inputEvent(e, changeGitUrl)}"/>
            <input type="text" id="git_branch" placeholder="Git Branch" on:change="{(e) => inputEvent(e, changeGitBranch)}"/>
        </div>
        <textarea type="text" id="project_description" placeholder="Project Description" on:change="{(e) => inputEvent(e, changeDescription)}" value="{data.p.description}" />
        <select id="project_type" on:change="{(e) => inputEvent(e, changeType)}">
            {#each data.targetPortfolio.types as type}
                {#if type == data.p.type}
                    <option value="{type}" selected>{type}</option>
                {:else}
                    <option value="{type}">{type}</option>
                {/if}
            {/each}
        </select>
        <p>
            <span>Create in: <i>{targetPathPreview}</i></span>
        </p>
        {#if configExists}
            <button id="create_project" on:click="{_ => editProject()}">Edit Project</button>
        {:else}
            <button id="create_project" on:click="{_ => createProject()}">Create Project</button>
        {/if}
    </div>
</div>

<style>
    #newproject_view {
        height: 100%;
		padding: 0 1rem;
		overflow-y: scroll;
        color: var(--font-color-light)
    }

    #top_wrapper {
        display: flex;
        flex-direction: row;
        justify-content: stretch;
        align-items: center;
        background-color: var(--primary-color);
        padding: 1rem;
    }

    #main_wrapper {
        padding: 1rem;
        height: 100%;
    }

    #project_name {
        width: 100%;
        height: 40px;
        padding: 0.5rem;
        margin: 0;
    }

    #project_description {
        width: 100%;
        height: 50%;
        padding: 0.5rem;
        margin: 0.5rem 0;
    }

    #project_type {
        color: var(--font-color-light);
    }

    #git {
        display: flex;
        flex-direction: row;
        justify-content: stretch;
        align-items: left;
    }

    #git_url {
        width: 100%;
        height: 40px;
        margin-right: 5px;
    }

    #git_branch {
        width: 30%;
        height: 40px;
    }

</style>