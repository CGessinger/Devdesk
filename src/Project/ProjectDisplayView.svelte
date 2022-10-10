<script lang="ts">
    import SvelteMarkdown from 'svelte-markdown'
    import type { ProjectModel } from "./utils/ProjectModel";
    import { terminal } from "$utils/Scripts";
    import { fs } from "$src/utils/Path";

    export let data: ProjectModel;
    let configExists = false;
    data.config_exists().then(exists => configExists = exists);

    function terminalHere() {
        terminal.terminal_here(data.path);
    }

    function vscodeHere() {
        terminal.vscode_here(data.path);
    }
</script>

<div class="mt-3 container">
    <h1 class="text-white">
        {data.name}
    </h1>
    <ol class="breadcrumb">
        {#each fs.splitPath(data.path) as part }
            <li class="breadcrumb-item text-muted">{part}</li>
        {/each}
    </ol>
    <div class="d-flex justify-content-start">
        <button class="btn btn-dark me-2" on:click="{terminalHere}">Terminal</button>
        <button class="btn btn-dark" on:click="{vscodeHere}">Editor</button>
    </div>
    <div class="text-white mt-3">
        <SvelteMarkdown source={data.description} />
    </div>
</div>

<style>
</style>