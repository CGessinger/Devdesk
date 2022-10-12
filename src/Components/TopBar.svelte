<script lang="ts">
    import { Modal } from "bootstrap"
    import { PortfolioModel } from "$src/Portfolio/utils/PortfolioModel";
    import { StateController, cached_settings } from "$src/store";
    import { emit } from "@tauri-apps/api/event";

    $: value = $StateController.value;
    $: prevValue = $StateController._prevValue;

    $: portfolio = ((): PortfolioModel => {
        if (value instanceof PortfolioModel) 
            return value;
        else if (prevValue instanceof PortfolioModel)
            return prevValue;

        return null;
    })()

    function focusType(i: number) {
        if (!portfolio)
            return;

        portfolio.focused_type = i;
        StateController.switchToPortfolio(portfolio);
    }

    function addType(e) {
        if (!portfolio)
            return;

        const inputObject = e.target.firstChild;
        console.log(inputObject);
        if (!inputObject || inputObject.value == "") 
            return;
        
        const input = inputObject.value;
        if(!portfolio.types.includes(input)) {
            const i = portfolio.types.push(input);
            portfolio.focused_type = i - 1;
            $cached_settings.safeSettings();
            StateController.switchToPortfolio(portfolio);
            inputObject.value = "";
        }
    }

    function removeType(e) {
        if (!portfolio)
            return;

        const targettedTypeIndex = portfolio.focused_type;
        portfolio.focused_type = targettedTypeIndex - 1;
        portfolio.types.splice(targettedTypeIndex, 1);
        $cached_settings.safeSettings();
        StateController.switchToPortfolio(portfolio);
    }

    function searchChange(e) {
        if (!portfolio)
            return;

        const value: String = e.target.value;
        emit('searchInputChange', value);
    }
</script>

<div class="top-bar-all mt-3 w-100">
    {#if portfolio}
        <form class="mb-3 d-flex">
            <input class="form-control text-bg-scheme me-2" type="text" placeholder="Search..." on:input="{searchChange}" />
            <button class="btn btn-scheme dropdown-toggle w-25 me-2" data-bs-toggle="dropdown" aria-expanded="false">{portfolio.getFocusedTypeString()}</button>
            <button class="btn btn-scheme" type="button" on:click="{removeType}">
                <i class="bi bi-trash"/>
            </button>
            <ul class="dropdown-menu dropdown-menu-dark text-bg-scheme">
                {#each portfolio.types as type, i}
                    <li class="dropdown-item text-on-dark" on:click={(_) => focusType(i)}>
                        {type}
                    </li>
                {/each}
                <li><hr class="dropdown-divider"></li>
                <li class="dropdown-item">
                    <form class="d-flex" on:submit|preventDefault="{addType}">
                        <input type="text" class="form-control text-bg-scheme me-1" placeholder="New Type"/>
                        <button class="btn btn-scheme-darker" type="submit">
                            <i class="bi bi-plus"/>
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
