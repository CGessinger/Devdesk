<script lang="ts">
    import { Settings } from "$utils/Data";

    export let data;
    data = null;

    let settings: Settings.Model = Settings.DefaultSettings;
    Settings.getSettings().then((s) => {
        settings = s;
    });

    function darkModeChange(e) {
        const darkMode = e.target.checked;
        if (darkMode) {
            document.documentElement.setAttribute('data-theme', 'dark');
        }
        else {
            document.documentElement.setAttribute('data-theme', 'light');
        }
        const newSettings = Settings.modelFrom(settings, {switches: {darkMode: darkMode}});
        Settings.changeSettings(newSettings);
    }

    function experimentalChange(e) {
        const experimental = e.target.checked;
        const newSettings = Settings.modelFrom(settings, {switches: {experimental: experimental}});
        Settings.changeSettings(newSettings);
    }

    function animationChange(e) {
        const animation = e.target.checked;
        const newSettings = Settings.modelFrom(settings, {switches: {runThree: animation}});
        Settings.changeSettings(newSettings);
    }

    function editorCommandChange (e) {
        const input = e.target.value;
        const newSettings = Settings.modelFrom(settings, {commands: {editorCmd: input}});
        Settings.changeSettings(newSettings);
    }
    
    function terminalCommandChange (e) {
        const input = e.target.value;
        const newSettings = Settings.modelFrom(settings, {commands: {terminalCmd: input}});
        Settings.changeSettings(newSettings);
    }

</script>

<div class="container mt-3">
    <h1 class="text-on-light mb-3">Settings</h1>

    <div class="container">
        <h3 class="text-on-light">General</h3>
        <div class="container">
            <div class="form-check form-switch">
                <input class="form-check-input bg-scheme border-white" 
                    type="checkbox" role="switch" id="flexSwitchCheckDefault"
                    data-for-prop="darkMode"
                    on:change={darkModeChange} checked="{settings.switches.darkMode}">
                <label class="form-check-label text-on-light" for="flexSwitchCheckDefault">Toggle Dark Mode!</label>
            </div>
            <div class="form-check form-switch">
                <input class="form-check-input bg-scheme border-white" 
                    type="checkbox" role="switch" id="flexSwitchCheckDefault" 
                    on:change={animationChange} checked="{settings.switches.runThree}">
                <label class="form-check-label text-on-light" for="flexSwitchCheckDefault">Toggle Animations</label>
            </div>
            <div class="form-check form-switch">
                <input class="form-check-input bg-scheme border-white" 
                    type="checkbox" role="switch" id="flexSwitchCheckDefault" 
                    on:change={experimentalChange} checked="{settings.switches.experimental}">
                <label class="form-check-label text-on-light" for="flexSwitchCheckDefault">Enable Experimental Features</label>
            </div>
        </div>
        <h3 class="text-on-light mt-3">Commands</h3>
        <div class="container">
            <div>
                <span class="text-on-light">Standard Editor Command</span>
                <input class="form-control w-50 text-bg-scheme" type="text" on:change="{editorCommandChange}" value="{settings.commands.editorCmd}"/>
            </div>
            <div class="mt-2">
                <span class="text-on-light">Standard Terminal Open Command</span>
                <input class="form-control w-50 text-bg-scheme" type="text" on:change="{terminalCommandChange}" value="{settings.commands.terminalCmd}"/>
            </div>
        </div>
    </div>

</div>

<style>
</style>
