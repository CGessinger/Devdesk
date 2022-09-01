<script lang="ts">
    import { cached_settings } from "$src/store";
    import type { Settings } from "$src/utils/Settings";

    let s: Settings;
    cached_settings.subscribe((value) => (s = value));

    function toggle_dark_mode() {
        s.dark_mode = !s.dark_mode;
        cached_settings.update((settings) => (settings = s));
        s.safe_settings();
    }
</script>

<div id="settings_view">
    <h1 id="settings_name">Settings</h1>

    <div class="theme-switch-wrapper">
        <label id="theme-switch" for="checkbox">
            <input
                type="checkbox"
                id="checkbox"
                checked={s.dark_mode}
                on:change={(_) => toggle_dark_mode()}
            />
            <div class="slider round" />
        </label>
        <em>Enable Dark Mode!</em>
    </div>
</div>

<style>
    #settings_view {
        padding: 0 1rem;
        overflow-y: scroll;
    }

    #settings_name {
        font-size: 2rem;
        font-weight: 600;
        margin: 0 0 1rem 0;
        text-align: center;
        color: var(--font-color-dark);
        background-color: var(--primary-color);
    }

    /*Simple css to style it like a toggle switch*/
    .theme-switch-wrapper {
        display: flex;
        align-items: center;
    }

    .theme-switch-wrapper em {
        margin-left: 10px;
        font-size: 1rem;
        color: var(--font-color-light);
    }

    #theme-switch {
        display: inline-block;
        height: 34px;
        position: relative;
        width: 60px;
    }

    #theme-switch input {
        display: none;
    }

    .slider {
        background-color: #ccc;
        bottom: 0;
        cursor: pointer;
        left: 0;
        position: absolute;
        right: 0;
        top: 0;
        transition: 0.4s;
    }

    .slider:before {
        background-color: var(--font-color-dark);
        bottom: 4px;
        content: "";
        height: 26px;
        left: 4px;
        position: absolute;
        transition: 0.4s;
        width: 26px;
    }

    input:checked + .slider {
        background-color: var(--primary-color);
    }

    input:checked + .slider:before {
        transform: translateX(26px);
    }

    .slider.round {
        border-radius: 34px;
    }

    .slider.round:before {
        border-radius: 50%;
    }
</style>
