<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { appWindow } from "@tauri-apps/api/window";
    import Breadcrump from "./lib/Breadcrump.svelte";
    import ProjectList from "./lib/ProjectList.svelte";
    import ProjectView from "./lib/ProjectView.svelte";
    import DefaultMain from "./lib/DefaultMain.svelte";
    import SearchInput from "./lib/SearchInput.svelte";
    import SubdirSelector from "./lib/SubdirSelector.svelte";
    import Clock from "./lib/Clock.svelte";
    import Logo from "./../assets/icon.ico";

    let current_vault = null;
    let breadcrumps = [];
    let current_projects = [];
    let current_subdirs = [];
    let recent_projects = [];
    let selected_id: number = null;
    invoke("get_init_info").then((info: any) => {
        console.log("get info");
        current_vault = info.vault;
        breadcrumps = current_vault?.path.split("/");
        current_projects = info.projects;
        current_subdirs = info.sub_directories;
        recent_projects = info.recent;
        window.info = info;
    });
    appWindow.listen("current_vault_change", (event) => {
        let info: any = event.payload;
        current_vault = info.vault;
        current_projects = info.projects;
        current_subdirs = info.sub_directories;
        recent_projects = info.recent;
        selected_id = info.selected_id;
        const project = current_projects.find(
            (p) => p.project_id == selected_id
        );
        const breadcrumpPath = project?.path || current_vault.path;
        breadcrumps = breadcrumpPath.split("/");
        window.info = info;
    });
</script>

<main>
    <div class="navbar">
        <Breadcrump path={breadcrumps} />
        <Clock />
        <a
            href="/"
            class="logo"
            on:click|preventDefault={(_) => invoke("reset_current_vault")}
        >
            <img src={Logo} class="logo" alt="Projector Logo" />
        </a>
    </div>
    <div class="left-panel">
        <SearchInput />
        <SubdirSelector
            subdirs={current_subdirs}
            on:go_back={(_) =>
                invoke("focus_vault", {
                    id: Math.max(current_vault.parent_vault_id, 1),
                })}
        />
        <ProjectList projects={current_projects} />
    </div>
    <div class="main-panel">
        {#if selected_id}
            <ProjectView projectId={selected_id} />
        {:else}
            <DefaultMain recent={recent_projects} />
        {/if}
    </div>
</main>

<style>
    main {
        position: relative;
        display: grid;
        grid-template-columns: minmax(350px, 1fr) 3fr;
        grid-template-rows: 3rem calc(100vh - 3rem);
    }

    .navbar {
        grid-row: 1;
        grid-column: 2;
        background-color: rgb(28, 28, 30);
        border-bottom: 1px solid rgb(44, 44, 46);
        padding: 0 1rem;
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 0.5rem;
    }

    .logo {
        height: 100%;
        aspect-ratio: 1;
        z-index: 2;
    }

    .left-panel {
        grid-row: 1 / span 2;
        height: 100vh;
        background: rgb(44, 44, 46);
        display: flex;
        box-sizing: border-box;
        flex-direction: column;
        align-items: flex-start;
        padding: 3rem 1.5rem;
        gap: 20px;
        border-right: 1px solid black;
    }

    .main-panel {
        grid-column: 2;
        grid-row: 2;
        background-color: rgb(28, 28, 30);
    }
</style>
