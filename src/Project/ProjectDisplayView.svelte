<script lang="ts">
    import SvelteMarkdown from 'svelte-markdown'
    import LanguageStatComponent from '$src/Components/LanguageStatsComponent.svelte'
    import type { ProjectModel } from "./utils/ProjectModel";
    import { terminal } from "$utils/Scripts";
    import { fs } from "$utils/Path";
    import { stats } from "$utils/Stats";

    //console.log(LanguageStatComponent);

    export let data: ProjectModel;
    let configExists = false;
    data.config_exists().then(exists => configExists = exists);
    
    let languageStats: [string, number][] = [];
    stats.get_language_stats(data.path)
        .then(res => {
            const total = res.total;
            for (const [key, value] of Object.entries(res.languages)) {
                languageStats = [...languageStats, [key, value / total]];
            }
        })
        .catch(e => console.log(e));

    function terminalHere() {
        terminal.terminal_here(data.path);
    }

    function vscodeHere() {
        terminal.vscode_here(data.path);
    }
</script>

<div class="mt-3 container display-grid h-100">
    <div class="main-display">
        <h1 class="text-on-light">
            {data.name}
        </h1>
        <ol class="breadcrumb me-auto">
            {#each fs.splitPath(data.path) as part }
                <li class="breadcrumb-item text-muted">{part}</li>
            {/each}
        </ol>
        <div class="d-flex justify-content-start">
            <button class="btn btn-scheme me-2" on:click="{terminalHere}">Terminal</button>
            <button class="btn btn-scheme" on:click="{vscodeHere}">Editor</button>
        </div>
        <div class="text-on-dark mt-3 description mb-auto">
            <SvelteMarkdown source={data.description} />
        </div>
    </div>
    <div class="right-bar h-100 w-100">
        <div class="language-stats bg-scheme mb-auto">
            <LanguageStatComponent {languageStats}/>
        </div>
    </div>
</div>

<style>
    .display-grid {
        display: grid;
        grid-template-columns: 75% 1fr;
    }

    .main-display {
        grid-column: 1;
        height:100%;
        display:flex;
        flex-direction:column;
    }

    .main-display > div {
        flex: 0 1 auto;
    }

    .main-display .description {
        flex: 1 1 auto;
    }

    .right-bar {
        grid-column: 2;
    }

    .language-stats {
        padding: 0.4rem;
        border-radius: 0.375rem;
    }
</style>