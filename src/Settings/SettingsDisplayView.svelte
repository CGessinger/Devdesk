<script lang="ts">
    import type { SettingsModel } from "./utils/SettingsModel";

    export let data: SettingsModel;

    let darkMode = data.dark_mode;
    $: {
        data.dark_mode = darkMode;
        if (darkMode) {
            document.documentElement.setAttribute('data-theme', 'dark');
        }
        else {
            document.documentElement.setAttribute('data-theme', 'light');
        } 
    }

    $: {
        data.safeSettings();
    }

    function editorCommandChange (e) {
        const input = e.target.value;
        data.editorCmd = input;
    }
    
    function terminalCommandChange (e) {
        const input = e.target.value;
        data.terminalCmd = input;
    }

</script>

<div class="container mt-3">
    <h1 class="text-on-light mb-3">Settings</h1>

    <div class="container">
        <h3 class="text-on-light">General</h3>
        <div class="container">
            <div class="form-check form-switch">
                <input class="form-check-input bg-scheme border-white" type="checkbox" role="switch" id="flexSwitchCheckDefault" bind:checked={darkMode}>
                <label class="form-check-label text-on-light" for="flexSwitchCheckDefault">Toggle Dark Mode!</label>
            </div>
            <div class="form-check form-switch">
                <input class="form-check-input bg-scheme border-white" type="checkbox" role="switch" id="flexSwitchCheckDefault" bind:checked={data.runThree}>
                <label class="form-check-label text-on-light" for="flexSwitchCheckDefault">Toggle Animations</label>
            </div>
            <div class="form-check form-switch">
                <input class="form-check-input bg-scheme border-white" type="checkbox" role="switch" id="flexSwitchCheckDefault" bind:checked={data.experimental}>
                <label class="form-check-label text-on-light" for="flexSwitchCheckDefault">Enable Experimental Features</label>
            </div>
        </div>
        <h3 class="text-on-light mt-3">Commands</h3>
        <div class="container">
            <div>
                <span class="text-on-light">Standard Editor Command</span>
                <input class="form-control w-50 text-bg-scheme" type="text" on:change="{editorCommandChange}" value="{data.editorCmd}"/>
            </div>
            <div class="mt-2">
                <span class="text-on-light">Standard Terminal Open Command</span>
                <input class="form-control w-50 text-bg-scheme" type="text" on:change="{terminalCommandChange}" value="{data.terminalCmd}"/>
            </div>
        </div>
    </div>

</div>

<style>
</style>
