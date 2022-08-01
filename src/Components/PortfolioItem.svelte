<script lang="ts">
    import type { Portfolio } from "../Utils/Portfolio";

    export let portfolio: Portfolio;
    let selected_type: number = null;

    function select_portfolio_type(i: number): void {
        selected_type = i;
    }
</script>

<li class="portfolio_item">
    <h3 class="portfolio_head">📖{portfolio.path}</h3>
    <ul class="type_list">
        {#each portfolio.types as type, i}
            {#if selected_type == i}
                <li class="selected_type">
                    ➡   {type}
                </li>
            {:else}
                <li on:click="{(e) => select_portfolio_type(i)}">
                    ↠   {type}
                </li>
            {/if}
        {/each}
    </ul>
</li>

<style>
    .portfolio_item {
        padding-top: 1rem;
        display: inline-block;
        width: 100%;
    }

    .portfolio_head {
        margin: 0;
        padding: 0;
		font-size: 1rem;
    }

	.selected_type {
		cursor: default;
		background-color: #fffffe5f;
        border-radius: 2px;
	}

    .type_list {
        list-style-type: none;
    }

    .type_list li {
        cursor: pointer;
    }
</style>
