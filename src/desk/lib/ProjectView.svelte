<script lang="ts">
    import { invoke } from "@tauri-apps/api";

    export let project: any = null;
</script>

<div class="wrapper">
    <h1>{project.name}</h1>
    <button on:click={(_) => invoke("editor_at", { path: project.path })}
        >Workon</button
    >
    <button on:click={(_) => invoke("terminal_at", { path: project.path })}
        >Terminal</button
    >
</div>

<style>
    .wrapper {
        padding: 2rem;
    }
</style>
