<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { appWindow } from "@tauri-apps/api/window";

    export let projects = [];
    let selected: number = null;

    function selectProject(id: number) {
        if (selected == id) {
            id = null;
        }
        invoke("focus_project", { id });
    }
    appWindow.listen("current_vault_change", (event) => {
        let info: any = event.payload;
        selected = info.selected_id;
    });
</script>

<div class="project-list">
    {#each projects as project}
        <div
            class="project-item"
            class:selected={project.project_id == selected}
            on:click={(_) => selectProject(project.project_id)}
        >
            <span class="name">{project.name}</span>
            <span class="last-open">{project.modified}</span>
        </div>
    {/each}
</div>

<style>
    .project-list {
        width: 100%;
        display: flex;
        flex-direction: column;
        overflow-y: scroll;
        flex: 1;
    }

    :global(.project-item.selected) {
        background-color: rgb(255, 45, 85);
    }

    :global(.project-item.selected > .last-open) {
        color: white;
    }

    .project-item {
        padding: 15px;
        display: grid;
        grid-template-columns: 1fr Auto;
        border-radius: 0.5em;
    }

    .project-item:hover:not(.selected) {
        background: rgb(72, 72, 74);
    }

    .project-item .name {
        font-weight: bold;
    }

    :global(.project-item .last-open) {
        font-size: 0.9rem;
        color: rgb(255, 45, 85);
    }
</style>
