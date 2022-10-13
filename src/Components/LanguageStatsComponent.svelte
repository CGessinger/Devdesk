<script lang="ts">
    export let languageStats: [string, number][] = [];
</script>

<div class="container ms-0">
    {#each languageStats as [key, value]}
  	  	{#if value > 0.03}
            <div class="language-container mb-2">
                <div class="text-nowrap text-on-dark">{key}</div>
            	<div class="progress w-100 mt-1 px-0">
            		<div class="progress-bar" style="width:{value * 100}%;"></div>
            	</div>
            </div>
        {/if}
    {/each}
</div>

<style>
	.language-container {
		display: grid;
		grid-template-columns: 1fr 1fr;
	}
</style>