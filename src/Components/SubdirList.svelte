<script lang="ts">
    import { StateController } from "$src/store";

    import { Portfolio } from "$src/utils/Data";
    import { fs } from "$utils/Path";

    export let activePortfolio: Portfolio.Model | null = null;

    let subdirs: string[] = [];
    $: {
        if (activePortfolio) {
            fs.read_dir(activePortfolio.path).then(res => {
                subdirs = res;
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
    <span class="subdir-item"
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
        border: 2px solid rgba(255, 255, 255, 0.1);
    }

    .subdir-item.active {
        background-color: rgba(255, 255, 255, 0.1);
    }
</style>