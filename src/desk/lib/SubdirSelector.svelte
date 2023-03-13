<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { createEventDispatcher } from "svelte";
    import { formatter } from "../../utils/formatter";

    export let subdirs = [];
    let limited: number = 6; // Decides if all subdirs are shown or not.
    const dispatch = createEventDispatcher();

    $: subdirs, (limited = 6);

    function subdirClick(id: number) {
        invoke("focus_vault", { id });
    }

    function showMore() {
        limited = undefined;
    }
</script>

<div>
    {#each subdirs.slice(0, limited) as dir}
        <button on:click={(_) => subdirClick(dir.vault_id)} title={dir.path}
            >📂 {formatter.formatName(formatter.fileNameFrom(dir.path))}
        </button>
    {/each}
    {#if limited && subdirs.length >= limited}
        <button class="control-btn" on:click={showMore}>More</button>
    {/if}
    {#if subdirs[0]?.parent_vault_id != 1}
        <button class="control-btn" on:click={(_) => dispatch("go_back")}
            >Back</button
        >
    {/if}
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

    .control-btn {
        font-weight: bold;
        text-align: center;
    }
</style>
