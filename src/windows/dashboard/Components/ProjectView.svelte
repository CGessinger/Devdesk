<script lang="ts">
    import type { Project } from "$src/utils/Project";
    export let project: Project;
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
        background-color: #912F40;
    }
</style>