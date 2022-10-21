<script lang="ts">
    import * as bootstrap from "bootstrap"
    import { ProjectModelBuilder } from "./utils/ProjectModelBuilder";
    import type { ProjectModel } from "./utils/ProjectModel";
    import { ProjectFileHandler } from "./utils/ProjectFileHandler";
    import { StateController } from "$src/store";
	import { onMount } from 'svelte';

    // Animations
    import { SpaceBackground } from "./animation/SpaceBackground"
    import { Settings } from "$src/utils/Data";
    
    let experimental = false;

    let canvas;
    Settings.getSwitches()
        .then((s) => {
            experimental = s.experimental;
        });

    onMount(() => {
        Settings.getSwitches()
        .then((s) => {
            const space = new SpaceBackground(canvas, s);
            const tick = space.setup();
            tick();
        });
    });

    // pass builder which might already have some values
    export let data: ProjectModelBuilder;
    $: targetPortfolio = $StateController._prevValue;

    async function submitProject(e) {
        const form = e.target;
        if (!form.checkValidity()) {
            return;
        }

        const formData = new FormData(form);
        const builder = new ProjectModelBuilder(data.parameters);
        for (const [key, value] of formData) {
            builder.parameters[key] = value.toString();
        }
        
        const projectBuilt = builder.build();
        if (projectBuilt.is_err()) {
            console.log("error: ", projectBuilt);
            return;
        }
        const fileHandler = new ProjectFileHandler(projectBuilt.unwrap() as ProjectModel);
        const exists = await builder.projectAlreadyExists();

        if(!exists) {
            await fileHandler.createConfigFolder();
        }
        await fileHandler.writeToConfig();
        // ToDo clone git here

        StateController.switchToPortfolio(targetPortfolio);
    }
</script>

<div class="grid h-100">
    <canvas bind:this="{canvas}" class="webgl w-100 h-100"></canvas>
    <div class="container mt-5">
        <form class="needs-validation w-75 mx-auto" on:submit|preventDefault="{submitProject}" novalidate>
            <div class="input-group flex-nowrap mb-3">
                <span class="input-group-text text-bg-scheme">Name</span>
                <input name="name" type="text" class="form-control text-bg-scheme" placeholder="Project Name" required>
            </div>

            {#if experimental}
            <div class="input-group mb-3">
                <span class="input-group-text text-bg-scheme"><i class="bi bi-github"/></span>
                <input name="git_url" class="col form-control text-bg-scheme" type="text" placeholder="GitHub Url"/>
                <span class="input-group-text text-bg-scheme"><i class="bi bi-bezier2"/></span>
                <input name="git_branch" class="form-control col text-bg-scheme" type="text" placeholder="Repository Branch"/>
            </div>
            {/if}

            <div class="input-group mb-3">
                <span class="input-group-text text-bg-scheme">Description</span>
                <textarea name="description" class="form-control text-bg-scheme"></textarea>
            </div>

            <div class="input-group mb-3">
            <select  name="type" class="form-select text-bg-scheme" required>
                {#each targetPortfolio.types as type}
                    <option value="{type}" selected="{type == data.parameters.type}">{type}</option>
                {/each}
            </select>
            <span class="input-group-text text-bg-scheme">Type</span>
            </div>

            <div class="position-relative mb-0">
                <button class="btn btn-scheme border-white" type="submit">Create Project</button>
            </div>

        </form>
    </div>
</div>

<style>
    .grid {
        display: grid;
    }

    .grid * {
        grid-column: 1;
        grid-row: 1;
    }

    .input-group input {
        background-color: black;
    }
</style>