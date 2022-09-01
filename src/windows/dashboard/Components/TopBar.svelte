<script lang="ts">
	import { focused_portfolio, focused_project } from "$src/windows/store";
    import type { Portfolio } from "$utils/Portfolio";
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

    let focus: Portfolio;
	focused_portfolio.subscribe(value => {
		focus = value;
	});

    function focus_type(i: number) {
        focused_project.update(pr => pr = undefined);
        focus.focused_type = i;
        focus.load_projects_from_type().then(() => focused_portfolio.update(p => p = p));
    }

    function add_type() {
        const input = document.getElementById("type_input") as HTMLInputElement;
        const name = input.value;
        if (name.length > 0 && !focus.types.includes(name)) {
            focus.types.push(name);
            input.value = "";
            dispatch('safe-settings');
            focused_portfolio.update(p => p = p);
        }
    }

    function remove_type(i: number) {
        focus.focused_type = i - 1;
        focus.types.splice(i, 1);
        dispatch('safe-settings');
        focused_portfolio.update(p => p = p);
    }
</script>

<div class="TopBar">
    <div id="search_nav">
        <input id="search_input" type="text" placeholder="Search..." />
        <button id="search_button" class="button">Search</button>
    </div>

    <div id="type_nav">
        {#if focus}
            <div class="type_item" on:click="{_ => focus_type(-1)}">
                All
                {#if "all" == focus.get_focused_type()}
                    <hr>
                {/if}
            </div>
            {#each focus.types as type, i}
                <div class="type_item" on:click="{_ => focus_type(i)}">
                    {type}
                    {#if type == focus.get_focused_type()}
                        <i class="fa fa-minus remove_type" on:click="{_ => remove_type(i)}"></i>
                        <hr>
                    {/if}
                </div>
            {/each}
            <input id="type_input" type="text" placeholder="Add type..." on:keypress="{e => { if(e.key == "Enter") add_type() }}"/>
            <button id="type_add" on:click="{_ => add_type()}">Add</button>
        {/if}
    </div>
</div>

<style>
    .TopBar {
        grid-row: 1;
        display: grid;
        grid-template-columns: 16em 1fr;
        top: 0;
        padding: 1rem;
        padding-bottom: calc(1rem - 5px);
        color: #40434E;
    }

    #search_nav {
        grid-column: 1;
        padding-top: 3px;
    }

    #search_input {
        width: 10em;
        float: left;
    }

    #search_button {
        width: 4.5em;
        margin-left: 0.5rem;
        float: left;
        grid-column: 1;
    }

    #type_nav {
        border: none;
        overflow-x: scroll;
        white-space: nowrap;
        grid-column: 2;
    }

    .type_item {
        display: inline-block;
        text-align: center;
        padding: 8px 10px;
        cursor: pointer;
    }

    .remove_type {
        cursor: pointer;
        color: #912F40;
        padding-left: 2px;
    }

    .remove_type:hover {
        color: #cb5c6e;
    }

</style>