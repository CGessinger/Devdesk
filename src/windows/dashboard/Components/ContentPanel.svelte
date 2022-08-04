<script lang="ts">
	import type { Portfolio } from "../../../Utils/Portfolio";
	import { WebviewWindow, appWindow } from '@tauri-apps/api/window'
	import { emit } from '@tauri-apps/api/event'

	export let focus: Portfolio;

	async function add_project() {
		const webview = new WebviewWindow('theUniqueLabel', {
			url: '../../project-creation-index.html',
			width: 400,
			height: 600,
			alwaysOnTop: true,
			title: 'Create A New Project',
		});

		webview.once('request_portfolio', function () {
			emit("project_portfolio", focus);
		});

		webview.once("create_project", e => {
			console.log("receiving event", e.payload);
		});
	}
</script>

<div id="portfolio_view">
    <ul>
		{#each focus.projects as project}
			<li>{project}</li>
		{/each}
    </ul>
	<button id="add_project" class="fa" on:click="{_ => add_project()}">&#xf067;</button>
</div>

<style>
	#portfolio_view {
		height: 100%;
	}

	#portfolio_view ul {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	#add_project {
		display: inline-block;
		position: fixed;
		border: none;
		cursor: pointer;
		bottom: 0;
		right: 0;
		padding: 1rem;
		margin: 1rem;
		background-color: #912F40;
		color: whitesmoke;
	}
</style>