<script lang="ts">
    import { Model } from "$src/Project/Create/ProjectCreateModel";

    export let edit;

    let model = new Model(edit);
	let ViewData = model.GetViewData();
	model.onViewDataChange = (_) => {
		ViewData = model.GetViewData();
	}
    
    async function on_type_change(e) {
        model.change_type(e.target.value);
    }

    async function on_name_change(e) {
        model.change_name(e.target.value);
    }

    async function on_git_url_change(e) {
        model.change_git(e.target.value, null);
    }

    async function on_git_branch_change(e) {
        model.change_git(null, e.target.value);
    }

    async function on_description_change(e) {
        model.change_description(e.currentTarget.value);
    }
</script>

<div id="newproject_view">
    <div id="top_wrapper">
        <div>
            <img id="thumbnail" src="data:image/png;base64, {model.builder.p.image}" alt="P" on:click="{_ => model.change_icon()}" />
        </div>
        <input type="text" id="project_name" placeholder="Project Name" on:input="{on_name_change}" value="{ViewData["name"]}"/>
    </div>
    <div id="main_wrapper">
        <div id="git">
            <input type="text" id="git_url" placeholder="Git URL" on:change="{on_git_url_change}"/>
            <input type="text" id="git_branch" placeholder="Git Branch" on:change="{on_git_branch_change}"/>
        </div>
        <textarea type="text" id="project_description" placeholder="Project Description" on:change="{on_description_change}" value="{ViewData["description"]}" />
        <select id="project_type" on:change="{on_type_change}">
            {#each ViewData["types"] as type}
                {#if type == ViewData["focused_type"]}
                    <option value="{type}" selected>{type}</option>
                {:else}
                    <option value="{type}">{type}</option>
                {/if}
            {/each}
        </select>
        <p>
            <span>Create in: <i>{ViewData["path_preview"]}</i></span>
        </p>
        {#if ViewData["config_exists"]}
            <button id="create_project" on:click="{_ => model.edit_project()}">Edit Project</button>
        {:else}
            <button id="create_project" on:click="{_ => model.create_project()}">Create Project</button>
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

    #thumbnail {
        width: 40px;
        height: 40px;
        cursor: pointer;
    }

    #project_name {
        width: 100%;
        height: 40px;
        padding: 0.5rem;
        margin: 0 0 0 0.5rem;
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