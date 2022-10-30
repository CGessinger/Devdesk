<script lang="ts">
    import { ProjectModelBuilder } from "$src/Project/utils/ProjectModelBuilder";
    import { StateController } from "$src/store";

	async function addProject() {
		const builder = new ProjectModelBuilder();
		StateController.switchToProjectCreation(builder);
	}
</script>

<button class="add-project btn btn-scheme" on:click={(_) => addProject()}>
    New Project <i class="bi bi-plus"/>
</button>

<style>
    .add-project {
        background-color: rgba(255, 255, 255, 0.1);
        border: 1px solid rgba(255, 255, 255, 0.2);
        margin: 0.5rem;
    }
</style>