<script lang="ts">
	import { StateController } from "$src/store";
    import imgUrl from "../assets/icon.ico";

	function focusSettings() {
		StateController.switchToSettings();
	}
</script>

<div class="wrapper container">
    <img class="app-icon" src="{imgUrl}" alt="P">
    <span>Portfolios</span>
    <i class="settings bi bi-gear" on:click={_ => focusSettings()}/>
</div>

<style>
    .wrapper {
        flex-basis: calc(100% / 8);
        display: flex;
        flex-direction: row;
        flex-wrap: nowrap;
        justify-content: flex-start;
        align-items: center;
        gap: 0.5rem;
    }

    .wrapper > span {
        font-size: 2.2rem;
        font-weight: bold;
        color: #fff;
        text-transform: capitalize;
    }

    .wrapper > .app-icon {
        width: 3.5rem;
        height: 3.5rem;
    }

    .wrapper > .settings {
        color: #fff;
        cursor: pointer;
        font-size: 1.5rem;
        margin-top: 0.3rem;
    }
</style>