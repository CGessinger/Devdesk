<script lang="ts">
	import { open } from '@tauri-apps/api/dialog'
    import { invoke } from '@tauri-apps/api/tauri';
    import type { Portfolio } from '$utils/Portfolio';
    import { focused_portfolio, new_project } from '$src/store';
    import { Project } from '$utils/Project';

    let builder = Project.Builder.get();
    let focus: Portfolio;
    focused_portfolio.subscribe((value) => {
        focus = value;
    });
    
    builder.withType(focus.focused_type == -1 ? focus.types[0] : focus.get_focused_type());

    function create_project() {
        const res = builder.tryBuildPath(focus.path);
        if (res.is_err()) {
            console.log("error: ", res);
            return;
        }
        const built = builder.build();
        if (built.is_err()) {
            console.log("error: ", built);
            return;
        }
        const fb = new Project.Folder(built.unwrap() as Project);
        fb.createConfigFolder().then(() => {
            fb.writeToConfig();
        });
        // emit("create_project", "ayoo");
        // appWindow.close();
    }

    async function change_icon() {
        const selected = await open({
            multiple: false,
            title: "Change Icon",
            filters: [
                { name: "Images", extensions: ["png", "jpg", "jpeg"] }
            ],
        });
        builder.withImageB64(await invoke("load_image", { path: selected.toString() }));
        builder = builder;
    }

    async function on_type_change(e) {
        builder.withType(e.target.value);
        builder = builder;
    }

    async function on_name_change(e) {
        builder.withName(e.target.value);
        builder = builder;
    }
</script>

<div id="newproject_view">
    <div id="top_wrapper">
        <div>
            <img id="thumbnail" src="data:image/png;base64, {builder.p.image}" alt="P" on:click="{_ => change_icon()}" />
        </div>
        <input type="text" id="project_name" placeholder="Project Name" on:change="{on_name_change}" />
    </div>
    <div id="main_wrapper">
        <textarea type="text" id="project_description" placeholder="Project Description" on:change="{e => builder.withDescription(e.currentTarget.value)}" />
        <select id="project_type" on:change="{on_type_change}">
            {#each focus.types as type}
                {#if type == focus.get_focused_type()}
                    <option value="{type}" selected>{type}</option>
                {:else}
                    <option value="{type}">{type}</option>
                {/if}
            {/each}
        </select>
        <p>
            <span>Create in: <i>{builder.target_path(focus.path)}</i></span>
        </p>
        <button id="create_project" on:click="{_ => create_project()}">Create Project</button>
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