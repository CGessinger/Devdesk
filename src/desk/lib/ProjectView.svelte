<script lang="ts">
    import SpotlightCustomScripts from "./SpotlightCustomScripts.svelte";
    import SpotlightReadMe from "./SpotlightReadMe.svelte";
    import SpotlightTitle from "./SpotlightTitle.svelte";
</script>

<div class="wrapper">
    <SpotlightTitle />
    <SpotlightReadMe />
    <SpotlightCustomScripts />
</div>

<style>
    .wrapper {
        height: 100%;
        padding: 40px;
    }
</style>
