<script lang="ts">
    import type { ProjectModel } from "../utils/ProjectModel";
    import { Model } from "./ProjectDisplayModel";

    export let project: ProjectModel;
    let model = new Model();
</script>

<div id="project_view">
    <h1 id="project_name">
        {project.name}
    </h1>
    <div id="project_description">
        {project.description}
    </div>
</div>

<style>
    #project_view {
		padding: 0 1rem;
        overflow-y: scroll;
    }

    #project_name {
        font-size: 2rem;
        font-weight: 600;
        margin: 0;
        text-align: center;
        color: white;
        background-color: var(--primary-color);
    }
</style>