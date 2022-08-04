<script lang="ts">
	import { open } from '@tauri-apps/api/dialog'
    import { emit, once } from '@tauri-apps/api/event'
    import { invoke } from '@tauri-apps/api/tauri';
    import { Portfolio } from '$utils/Portfolio';
    import { Project } from '$utils/Project';
    import { isErr } from '$utils/Result';

    const builder = Project.Builder.get();

    let focus: Portfolio = new Portfolio("");
    emit("request_portfolio");
    once('project_portfolio', (e) => {
        focus = Object.assign(new Portfolio(""), JSON.parse(e.payload as string));
        builder.withType(focus.focused_type == -1 ? focus.types[0] : focus.get_focused_type());
    });

    function create_project() {
        const res = builder.tryBuildPath(focus.path);
        if (isErr(res)) {
            console.log("error: ", res);
            return;
        }
        const built = builder.build();
        if (isErr(built)) {
            console.log("error: ", built);
            return;
        }
        console.log("success", built);
        emit("create_project", "ayoo");
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
    }
</script>

<div id="main">
    <div id="top_wrapper">
        <div>
            <img id="thumbnail" src="data:image/png;base64, {builder.p.image}" alt="P" on:click="{_ => change_icon()}" />
        </div>
        <input type="text" id="project_name" placeholder="Project Name" on:change="{e => builder.withName(e.currentTarget.value)}" />
    </div>
    <div id="main_wrapper">
        <textarea type="text" id="project_description" placeholder="Project Description" on:change="{e => builder.withDescription(e.currentTarget.value)}" />
        <select id="project_type" on:change="{e => builder.withType(e.currentTarget.value)}">
            {#each focus.types as type}
                <option value="{type}">{type}</option>
            {/each}
        </select>
        <p>
            <span>Create in: <i>{focus.path}</i></span>
        </p>
        <button id="create_project" on:click="{_ => create_project()}">Create Project</button>
    </div>
</div>

<style>
    #main {
        height: 100%;
    }

    #top_wrapper {
        display: flex;
        flex-direction: row;
        justify-content: stretch;
        align-items: center;
        background-color: #912F40;
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

</style>