<script lang="ts">
    import type { Portfolio } from "../Utils/Portfolio";

    export let focus: Portfolio;
    let focused_type: string = "all";

    function focus_type(f: string) {
        focused_type = f;
    }
</script>

<div class="TopBar">
    <input id="search_input" type="text" placeholder="Search..." />

    <div id="type_nav">
        {#if focus}
            <div class="type_item" on:click="{_ => focus_type("all")}">
                All
                {#if "all" == focused_type}
                    <hr>
                {/if}
            </div>
            {#each focus.types as type}
                <div class="type_item" on:click="{_ => focus_type(type)}">
                    {type}
                    {#if type == focused_type}
                        <hr>
                    {/if}
                </div>
            {/each}
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
</style>