<script lang="ts">
    import {Modal} from "bootstrap"
    import { PortfolioModel } from "$src/Portfolio/utils/PortfolioModel";
    import { StateController, cached_settings } from "$src/store";

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
</script>

<div class="top-bar-all mt-3 w-100">
    {#if portfolio}
        <form class="mb-3 d-flex">
            <input class="form-control text-bg-dark me-2" type="text" placeholder="Search..." />
            <button class="btn btn-dark dropdown-toggle w-25 me-2" data-bs-toggle="dropdown" aria-expanded="false">{portfolio.getFocusedTypeString()}</button>
            <button class="btn btn-dark" type="button" on:click="{removeType}">
                <i class="bi bi-trash"/>
            </button>
            <ul class="dropdown-menu dropdown-menu-dark">
                {#each portfolio.types as type, i}
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
