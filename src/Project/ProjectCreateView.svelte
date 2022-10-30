<script lang="ts">
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

        StateController.switchToPrev;
    }

    function onNameInput(e) {
        data.parameters.name = e.target.value
    }
</script>

<div class="grid h-100">
    <canvas bind:this="{canvas}" class="webgl w-100 h-100"></canvas>
    <div class="container mt-5">
        <form class="needs-validation mx-auto" on:submit|preventDefault="{submitProject}" novalidate>
            <div class="name-input flex-nowrap mb-3">
                <span>Name</span>
                <input name="name" type="text" class="form-control text-bg-scheme"
                on:input={onNameInput} required>
            </div>

            {#if experimental}
            <div class="input-group mb-3">
                <span class="input-group-text text-bg-scheme"><i class="bi bi-github"/></span>
                <input name="git_url" class="col form-control text-bg-scheme" type="text" placeholder="GitHub Url"/>
                <span class="input-group-text text-bg-scheme"><i class="bi bi-bezier2"/></span>
                <input name="git_branch" class="form-control col text-bg-scheme" type="text" placeholder="Repository Branch"/>
            </div>
            {/if}

            <div class="description-input mb-3">
                <span>Description</span>
                <textarea name="description" class="form-control text-bg-scheme"></textarea>
            </div>

            <div class="input-group mb-3 text-bg-scheme input-group-text">
                <span>{data.getPathFormattedPortfolio().value}</span>
            </div>

            <div class="submit-wrapper position-relative mb-0">
                <button class="btn btn-scheme" type="submit">Create Project</button>
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

    form {
        width: 60%;
    }

    .input-group input {
        background-color: black;
    }

    .name-input,
    .description-input {
        position: relative;
    }

    .name-input > span,
    .description-input > span {
        position: absolute;
        color: white;
        top: 10px;
        left: 12px;
        font-weight: 300;
    }

    .description-input > textarea {
        height: 200px;
        padding-top: calc(1rem + 24px);
    }

    .name-input > input {
        padding-top: calc(1rem + 24px);
        left: 10px;
    }

    .submit-wrapper {
        background-color: var(--bg-front-color);
        display: inline-block;
        border-radius: 0.375rem;
    }

    button[type="submit"] {
        color: white;
        background-color: rgba(255, 255, 255, 0.1);
        border: 1px solid rgba(255, 255, 255, 0.2);
    }
</style>