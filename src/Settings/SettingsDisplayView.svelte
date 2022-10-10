<script lang="ts">
    import type { SettingsModel } from "./utils/SettingsModel";

    export let data: SettingsModel;
    let darkMode = data.dark_mode;
    let runAnimations = data.runThree;

    $: {
        data.dark_mode = darkMode;
        data.runThree = runAnimations;
        data.safeSettings();
    }

</script>

<div class="container mt-3">
    <h1 class="text-white mb-3">Settings</h1>

    <div class="container">
        <div class="form-check form-switch">
            <input class="form-check-input bg-dark border-white" type="checkbox" role="switch" id="flexSwitchCheckDefault" bind:checked={darkMode}>
            <label class="form-check-label text-white" for="flexSwitchCheckDefault">Toggle Dark Mode!</label>
        </div>
        <div class="form-check form-switch">
            <input class="form-check-input bg-dark border-white" type="checkbox" role="switch" id="flexSwitchCheckDefault" bind:checked={runAnimations}>
            <label class="form-check-label text-white" for="flexSwitchCheckDefault">Toggle Animations</label>
        </div>
    </div>

</div>

<style>
</style>
