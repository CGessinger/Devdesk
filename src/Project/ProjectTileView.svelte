<script lang="ts">
    export let project;
</script>


<div class="project_item text-bg-dark">
    <div class="project_name">
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