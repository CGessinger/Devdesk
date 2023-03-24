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
    import * as deskapi from "../utils/deskapi";

    import type { Dashboard } from "../utils/types";
    import { formatter } from "../utils/formatter";
    import Taskbar from "./lib/Taskbar.svelte";

    let dashboard: Dashboard = null;
    let breadcrumps: string[] = [];
    invoke("get_init_info").then((info: Dashboard) => {
        dashboard = info;
        breadcrumps = formatter.breadcrumpsFrom(
            dashboard.selected?.path,
            dashboard.vault.path
        );
    });
    appWindow.listen("current_vault_change", (event) => {
        let info: Dashboard = event.payload;
        dashboard = info;
        breadcrumps = formatter.breadcrumpsFrom(
            dashboard.selected?.path,
            dashboard.vault.path
        );
    });
</script>

<main>
    <div class="navbar">
        <Breadcrump />
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
            subdirs={dashboard?.sub_directories}
            on:go_back={(_) =>
                invoke("focus_vault", {
                    id: Math.max(dashboard?.vault.parent_vault_id, 1),
                })}
        />
        <ProjectList projects={dashboard?.projects} />
    </div>
    <div class="taskbar">
        <Taskbar />
    </div>
    <div class="main-panel">
        {#if dashboard?.selected}
            <ProjectView />
        {:else}
            <DefaultMain recent={dashboard?.recent} />
        {/if}
    </div>
</main>

<style>
    main {
        position: relative;
        display: grid;
        grid-template-columns: minmax(350px, 1fr) 3fr;
        grid-template-rows: 3rem calc(100vh - 5rem) 2rem;
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
        height: 100%;
        background: rgb(44, 44, 46);
        display: flex;
        box-sizing: border-box;
        flex-direction: column;
        align-items: flex-start;
        padding: 3rem 1rem 1rem 1rem;
        border-right: 1px solid black;
        gap: 0.5rem;
    }

    .taskbar {
        grid-row: 3;
        grid-column: 1;
        background-color: rgb(69, 123, 157);
        width: 100%;
        border-top: 1px solid rgb(44, 44, 46);
        align-items: center;
        gap: 0.5rem;
        border-right: 1px solid black;
    }

    .main-panel {
        grid-column: 2;
        grid-row: 2 / span 2;
        background-color: rgb(28, 28, 30);
    }
</style>
