<script lang="ts">
    const types = ["Concept", "Sandbox", "Release"];

    export let portfolio: string = "";
    let selected_type: number = null;

    function select_portfolio_type(i: number): void {
        selected_type = i;
    }
</script>

<div>
    <h3>{portfolio}</h3>
    <ul>
        {#each types as type, i}
            {#if selected_type == i}
                <li class="selected_type">
                    {type}
                </li>
            {:else}
                <li on:click="{(e) => select_portfolio_type(i)}">
                    {type}
                </li>
            {/if}
        {/each}
    </ul>
</div>

<style>
	.selected_type {
		cursor: default;
		background-color: green;
	}
</style>
