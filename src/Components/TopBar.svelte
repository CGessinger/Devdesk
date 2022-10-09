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
        const inputObject = e.target.firstChild.firstChild;
        if (!inputObject || inputObject.value == "") 
            return;
        
        const input = inputObject.value;
        if(!focus.types.includes(input)) {
            focus.types.push(input);
            $cached_settings.safeSettings();
            focused_portfolio.update((p) => (p = p));
            inputObject.value = "";
            const modalCloseButton = document.getElementById('new-type-modal-close');
            modalCloseButton.click();
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

<div class="top-bar-all mt-3">
    {#if focus}
        <form class="mb-3">
            <input class="form-control text-bg-dark" type="text" placeholder="Search..." />
        </form>
        <div class="d-flex justify-content-center mb-3">
            <ul class="nav justify-content-center pt-pb-1 ms-auto overflow-auto">
                <li class="nav-link text-white" on:click={(_) => focusType(-1)}>All</li>
                {#each focus.types as type, i}
                    <li class="nav-link text-white" on:click={(_) => focusType(i)}>
                        {type}
                    </li>
                {/each}
            </ul>
            <div class="text-white ms-auto me-1">
                <button class="btn btn-dark" type="button" data-bs-toggle="modal" data-bs-target="#new-type-modal">
                    <i class="bi bi-plus text-white"/>
                </button>
            </div>
            <div class="text-white">
                <button class="btn btn-dark" type="button" on:click="{removeType}">
                    <i class="bi bi-trash"/>
                </button>
            </div>
        </div>
    {/if}
</div>
<div class="modal" id="new-type-modal" tabindex="-1">
    <div class="modal-dialog">
      <div class="modal-content text-bg-dark">
        <div class="modal-header">
          <h5 class="modal-title">New Type</h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <form on:submit|preventDefault="{addType}">
            <div class="modal-body">
                <input class="form-control" type="text" id="type-input" placeholder="New Type" aria-label="Type">
            </div>
        <div class="modal-footer">
            <button type="submit" class="btn btn-primary">Save changes</button>
            <button type="button" id="new-type-modal-close" class="btn btn-secondary" data-bs-dismiss="modal">Close</button>
        </div>
        </form>
      </div>
    </div>
  </div>

<style>
    .top-bar-all {
        height: 7rem;
    }

    .nav {
        height: 3rem;
    }

    .nav-link {
        cursor: pointer;
    }
</style>
