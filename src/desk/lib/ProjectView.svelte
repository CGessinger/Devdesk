<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import SvelteMarkdown from "svelte-markdown";
    import { formatter } from "../../utils/formatter";

    // export let project: any = null;
    export let projectId: number;
    let readme = "";
    let scripts = [];
    $: {
        invoke("get_project_view", { projectId }).then((info: any) => {
            readme = info.readme || `# ${info.name}`;
            scripts = info.scripts;
        });
    }

    function executeScriptByName(scriptName: string) {
        invoke("execute_script_by_name", { scriptName, projectId });
    }
</script>

<div class="wrapper">
    <div class="head-wrapper">
        <span style="--tag-bg: #B7410E">Tags coming soon here!</span>
        <!-- <span style="--tag-bg: #B7410E">Rust</span>
        <span style="--tag-bg: #3178C6">Typescript</span> -->
    </div>
    <div class="md-wrapper">
        <SvelteMarkdown source={readme} />
    </div>
    <div class="foot-wrapper">
        <!-- <button on:click={(_) => invoke("editor_at", { path: project.path })}
            >Workon</button
        >
        <button on:click={(_) => invoke("terminal_at", { path: project.path })}
            >Terminal</button
        > -->
        {#each scripts as script}
            <button
                title={script[1]}
                on:click={(_) => executeScriptByName(script[0])}
                >{formatter.formatScriptName(script[0])}</button
            >
        {/each}
    </div>
</div>

<style>
    .wrapper {
        height: 100%;
        padding: 40px;
        display: grid;
        grid-template-rows: auto 1fr auto;
        grid-template-columns: 75% 25%;
        gap: 20px;
    }

    .head-wrapper {
        display: flex;
        flex-direction: row;
        gap: 0.35em;
        grid-row: 1;
    }

    .head-wrapper span {
        display: inline-block;
        border-radius: 3px;
        padding: 0.2em 0.5em 0.3em;
        border-radius: 5px;
        background: var(--tag-bg);
        color: white;
        font-weight: 600;
    }

    .md-wrapper {
        grid-row: 2;
        overflow-y: scroll;
    }

    :global(.md-wrapper h1) {
        margin: 0;
    }

    .foot-wrapper {
        grid-row: 3;
    }

    .foot-wrapper > button {
        text-transform: capitalize;
    }
</style>
