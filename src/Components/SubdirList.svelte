<script lang="ts">
    import { StateController } from "$src/store";

    import { Portfolio } from "$src/utils/Data";
    import { projectdb } from "$src/utils/Database";
    import { fs } from "$utils/Path";

    export let activePortfolio: Portfolio.Model | null = null;

    let subdirs: string[] = [];
    $: {
        // Maybe there is a better solution to this?
        // Get all direct subfolders of the active portfolio
        if (activePortfolio) {
            const query = new projectdb.query();
            query.withDir(activePortfolio.path);
            query.foldersOnly();
            projectdb.query_database(query.build()).then((res) => {
                subdirs = [];
                const targetDir = fs.splitPath(activePortfolio.path).at(-1);
                res.forEach(dirEntry => {
                    const pathParts = fs.splitPath(dirEntry.path);
                    if (pathParts.at(-2) == targetDir) {
                        subdirs = [...subdirs, pathParts.at(-1)];
                    }
                });
            });
        }
    };

    function focusType(type: string) {
        if (!activePortfolio)
            return;
            
        const newPortfolio = Portfolio.modelFrom(activePortfolio, {subDirFilter: [type]});
        StateController.switchToPortfolio(newPortfolio);
    }
</script>

<div class="subdir-container">
    {#each subdirs as subdir}
    <span class="subdir-item btn-scheme"
    class:active={activePortfolio?.subDirFilter[0]?.toUpperCase() == subdir.toUpperCase()}
    on:click={_ => focusType(subdir)}>
        {subdir}
    </span>  
    {/each}
</div>

<style>
    .subdir-container {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: flex-start;
        align-items: center;
        align-content: flex-start;
        gap: 0.5rem;
    }

    .subdir-item {
		cursor: pointer;
        font-size: 0.8rem;
        border-radius: 2em;
        display: inline-block;
        padding: 0.2em 0.5em;
    }

    .subdir-item.active {
        background-color: #78002e;
    }
</style>