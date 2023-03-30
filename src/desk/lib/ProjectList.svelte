<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { appWindow } from "@tauri-apps/api/window";
    import type { Node } from "../../utils/deskapi";
    import { formatter } from "../../utils/formatter";

    let projects: Node[] = [];

    invoke("get_all").then((event) => {
        const nodes = event as Node[];
        projects = nodes.filter((n) => n.project);
        console.log(nodes);
    });
</script>

<div class="project-list">
    {#each projects as project}
        <button class="project-item">
            <span class="name">{formatter.formatName(project.name)}</span>
            <span class="last-open">{project.modified}</span>
        </button>
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

    .project-item.selected {
        background: rgb(255, 45, 85);
    }

    .project-item.selected > .last-open {
        color: white;
    }

    .project-item {
        background: unset;
        height: unset;
        font-size: unset;
        text-align: left;
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
        text-transform: capitalize;
    }

    .project-item .last-open {
        font-size: 0.9rem;
        color: rgb(255, 45, 85);
    }
</style>
