<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { createEventDispatcher } from "svelte";
    import { formatter } from "../../utils/formatter";

    export let subdirs = [];
    const dispatch = createEventDispatcher();

    function subdirClick(id: number) {
        invoke("focus_vault", { id });
    }
</script>

<div>
    {#each subdirs as dir}
        <button on:click={(_) => subdirClick(dir.vault_id)} title={dir.path}
            >📂 {formatter.formatName(dir.path.split("/").at(-1))}
        </button>
    {/each}
    <button on:click={(_) => dispatch("go_back")}>Back</button>
</div>

<style>
    div {
        width: 100%;
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 0.5rem;
        flex-flow: wrap;
    }

    button {
        height: unset;
        text-transform: capitalize;
        text-align: left;
        overflow: hidden;
        white-space: nowrap; /* Don't forget this one */
        text-overflow: ellipsis;
    }
</style>
