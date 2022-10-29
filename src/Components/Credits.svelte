<script lang="ts">
	import { getVersion } from '@tauri-apps/api/app';

	let appVersion = "";
	getVersion().then((v) => {
		appVersion = v;
	});
</script>

<div class="color-accent">
    <span class="text-on-dark text-decoration-none container">
        Made By
        <a class="text-on-dark text-decoration-none fw-bold" href="https://www.github.com/CGessinger"> CGessinger</a>
        <span>({appVersion})</span>
    </span>
</div>

<style>
	.color-accent {
		background-color: #78002e;
	}
</style>