<script lang="ts">
    export let project;

</script>


<div class="project_item">
    <div class="project_name">
        {#if project.image != ""}
            <img
                async
                class="project_logo"
                src="data:image/png;base64, {project.image}"
                alt={project.name}
            />
        {/if}
        <span>{project.name}</span>
    </div>
    <div class="project_git">
        <i class="bi bi-eye"/>
        <i class="bi bi-bezier2" />
        <i class="bi bi-star"/>
    </div>
    <div class="project_path">{project.path}</div>
</div>

<style>
	.project_item {
		display: grid;
		grid-template-areas:
			"header header header header git git"
			"main main main main right right"
			"footer footer footer footer footer footer";
		background-color: var(--primary-color);
		color: whitesmoke;
		margin-bottom: 1rem;
		padding: 10px;
		gap: 10px;
		cursor: pointer;
	}

	.project_name {
		grid-area: header;
		font-size: 1.5rem;
		font-weight: bold;
	}

	.project_logo {
		width: 32px;
		height: 32px;
	}

	.project_git {
		grid-area: git;
		text-align: right;
		fill: whitesmoke;
	}

	.project_path {
		grid-area: main;
		font-size: 0.5rem;
	}
</style>