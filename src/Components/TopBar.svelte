<script lang="ts">
    import {Modal} from "bootstrap"
    import type { PortfolioModel } from "$src/Portfolio/utils/PortfolioModel";
    import {
        focused_portfolio,
        focused_project,
        focus_settings,
        new_project,
        cached_settings
    } from "$src/store";

    let focus: PortfolioModel;
    focused_portfolio.subscribe((value) => {
        focus = value;
    });

    function focusType(i: number) {
        focused_project.update((pr) => (pr = undefined));
        focus_settings.update((fs) => (fs = false));
        new_project.update((np) => (np = undefined));
        focus.focused_type = i;
        focused_portfolio.update((p) => (p = p));
    }

    function addType(e) {
        const inputObject = e.target.firstChild;
        console.log(inputObject);
        if (!inputObject || inputObject.value == "") 
            return;
        
        const input = inputObject.value;
        if(!focus.types.includes(input)) {
            const i = focus.types.push(input);
            focus.focused_type = i-1;
            $cached_settings.safeSettings();
            focused_portfolio.update((p) => (p = p));
            inputObject.value = "";
        }
    }

    function removeType(e) {
        const targettedTypeIndex = focus.focused_type;
        focus.focused_type = targettedTypeIndex - 1;
        focus.types.splice(targettedTypeIndex, 1);
        $cached_settings.safeSettings();
        focused_portfolio.update((p) => (p = p));
    }
</script>

<div class="top-bar-all mt-3 w-100">
    {#if focus}
        <form class="mb-3 d-flex">
            <input class="form-control text-bg-dark me-2" type="text" placeholder="Search..." />
            <button class="btn btn-dark dropdown-toggle w-25 me-2" data-bs-toggle="dropdown" aria-expanded="false">{focus.get_focused_type()}</button>
            <button class="btn btn-dark" type="button" on:click="{removeType}">
                <i class="bi bi-trash"/>
            </button>
            <ul class="dropdown-menu dropdown-menu-dark">
                {#each focus.types as type, i}
                    <li class="dropdown-item text-white" on:click={(_) => focusType(i)}>
                        {type}
                    </li>
                {/each}
                <li><hr class="dropdown-divider"></li>
                <li class="dropdown-item">
                    <form class="d-flex" on:submit|preventDefault="{addType}">
                        <input type="text" class="form-control text-bg-dark me-1" placeholder="New Type"/>
                        <button class="btn btn-dark" type="submit">
                            <i class="bi bi-plus text-white"/>
                        </button>
                    </form>
                </li>
            </ul>
        </form>
    {/if}
</div>

<style>
    .top-bar-all {
        height: 3rem;
    }

    .dropdown-item {
        cursor: pointer;
    }
</style>
