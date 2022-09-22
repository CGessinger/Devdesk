<script lang="ts">
    import { Model } from "$src/Project/Create/ProjectCreateModel";

    let model = new Model();
	let ViewData = model.GetViewData();
	model.onViewDataChange = (_) => {
		ViewData = model.GetViewData();
	}
    
    async function on_type_change(e) {
        model.builder.withType(e.target.value);
        model = model;
    }

    async function on_name_change(e) {
        model.builder.withName(e.target.value);
        model = model;
    }
</script>

<div id="newproject_view">
    <div id="top_wrapper">
        <div>
            <img id="thumbnail" src="data:image/png;base64, {model.builder.p.image}" alt="P" on:click="{_ => model.change_icon()}" />
        </div>
        <input type="text" id="project_name" placeholder="Project Name" on:change="{on_name_change}" />
    </div>
    <div id="main_wrapper">
        <textarea type="text" id="project_description" placeholder="Project Description" on:change="{e => model.builder.withDescription(e.currentTarget.value)}" />
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
            <span>Create in: <i>{model.builder.target_path(ViewData["path_preview"])}</i></span>
        </p>
        <button id="create_project" on:click="{_ => model.create_project()}">Create Project</button>
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

</style>