<script lang="ts">
    import type { Portfolio } from "../../../Utils/Portfolio";
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

    export let focus: Portfolio;

    function focus_type(i: number) {
        if (focus.focused_type == i)
            return;

        focus.focused_type = i;
        focus.load_projects_from_type().then(() => dispatch('refresh-focus'));
    }

    function add_type() {
        const input = document.getElementById("type_input") as HTMLInputElement;
        const name = input.value;
        if (name.length > 0 && !focus.types.includes(name)) {
            focus.types.push(name);
            input.value = "";
            dispatch('safe-settings');
            dispatch('refresh-focus')
        }
    }

    function remove_type(i: number) {
        focus.focused_type = i - 1;
        focus.types.splice(i, 1);
        dispatch('safe-settings');
        dispatch('refresh-focus')
    }
</script>

<div class="TopBar">
    <input id="search_input" type="text" placeholder="Search..." />
    <button id="search_button" class="button">Search</button>

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
        position: relative;
        top: 0;
        padding: 1rem;
        color: #40434E;
        width: 100%;
    }

    #search_input {
        width: 20%;
    }

    #type_nav {
        display: inline-block;
        position: absolute;
        top: 0;
        padding: 1rem;
        border: none;
        cursor: pointer;
    }

    .type_item {
        float: left;
        text-align: center;
        padding: 8px 10px;
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