<script lang="ts">
    import SvelteMarkdown from 'svelte-markdown'
    import LanguageStatComponent from '$src/Components/LanguageStatsComponent.svelte'
    import { Offcanvas } from "bootstrap"
    import type { ProjectModel } from "./utils/ProjectModel";
    import { terminal } from "$utils/Scripts";
    import { fs } from "$utils/Path";
    import { stats } from "$utils/Stats";
    import { Settings } from '$utils/Data';

    export let data: ProjectModel;
    let configExists = false;
    data.config_exists().then(exists => configExists = exists);

    let makefileExists = false;
    fs.makefile_exists(data.path).then(res => makefileExists = res);

    let settings: Settings.Model = Settings.DefaultSettings;
    let languageStats: [string, number][] = [];
    Settings.getSettings().then(s => {
        settings = s;
        
        if (settings.switches.experimental) {
            stats.get_language_stats(data.path)
                .then(res => {
                    languageStats = [];
                    const total = res.total;
                    for (const [key, value] of Object.entries(res.languages)) {
                        languageStats = [...languageStats, [key, value / total]];
                    }
                })
                .catch(e => console.log(e));
        }
    });
    

    let terminalOutput = {
        stdout: "",
        stderr: "" 
    }
    async function makeHere() {
        const result = await terminal.make_here(data.path);
        terminalOutput.stdout = result[0];
        terminalOutput.stderr = result[1];
        const offcanvas = new Offcanvas('#staticBackdrop');
        offcanvas.toggle();
    }

    function editorHere() {
        terminal.editorHere(data.path, settings.commands.editorCmd).catch(e => {
            terminalOutput.stdout = "";
            terminalOutput.stderr = e;
            const offcanvas = new Offcanvas('#staticBackdrop');
            offcanvas.toggle();
        }).then(res => {
            if(res[1]) {
                terminalOutput.stdout = res[0];
                terminalOutput.stderr = res[1];
                const offcanvas = new Offcanvas('#staticBackdrop');
                offcanvas.toggle();
            }
        });
    }

    function terminalHere() {
        terminal.terminal_here(data.path, settings.commands.terminalCmd).catch(e => {
            terminalOutput.stdout = "";
            terminalOutput.stderr = e;
            const offcanvas = new Offcanvas('#staticBackdrop');
            offcanvas.toggle();
        }).then(res => {
            if(res[1]) {
                terminalOutput.stdout = res[0];
                terminalOutput.stderr = res[1];
                const offcanvas = new Offcanvas('#staticBackdrop');
                offcanvas.toggle();
            }
        });
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
        <div class="d-flex justify-content-start gap-3">
            <button class="btn btn-scheme" on:click="{terminalHere}">Terminal</button>
            <button class="btn btn-scheme" on:click="{editorHere}">Editor</button>
            {#if settings.switches.experimental && makefileExists}
                <button class="btn btn-scheme" on:click="{makeHere}">Make</button>
            {/if}
        </div>
        <div class="text-on-dark mt-3 description mb-auto">
            <SvelteMarkdown source={data.description} />
        </div>
    </div>
    <div class="right-bar h-100 w-100">
        {#if settings.switches.experimental}
            <div class="language-stats bg-scheme mb-auto">
                <LanguageStatComponent {languageStats}/>
            </div>
        {/if}
    </div>
</div>
<div class="offcanvas offcanvas-bottom text-bg-scheme" tabindex="-1" id="staticBackdrop" aria-labelledby="staticBackdropLabel">
  <div class="offcanvas-header">
    <h5 class="offcanvas-title" id="staticBackdropLabel">Terminal</h5>
    <button type="button" class="btn-close text-on-dark" data-bs-dismiss="offcanvas" aria-label="Close"></button>
  </div>
  <div class="offcanvas-body">
    <div class="container">
        Output: 
        <p class="w-100 p-3 terminal-text">
            <code>{terminalOutput.stdout}</code>
        </p>
        Error: 
        <p class="w-100 p-3 terminal-text">
            <code>{terminalOutput.stderr}</code>
        </p>
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
        overflow-y: scroll;
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