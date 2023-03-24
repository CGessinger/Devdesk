<script lang="ts">
    import SvelteMarkdown from "svelte-markdown";
    import { fileExists, readFile } from "../../utils/deskapi";
    import { formatter } from "../../utils/formatter";

    let readme = "";

    window.addEventListener("spotlight_changed", async (data: CustomEvent) => {
        const readmePath = formatter.appendToPath(
            data.detail.path,
            "README.md"
        );
        const exists = await fileExists(readmePath);
        if (exists) {
            readme = await readFile(readmePath);
        }
    });
</script>

<div class="md-wrapper">
    <div class="no-events-wrapper">
        <SvelteMarkdown source={readme} />
    </div>
</div>

<style>
    .md-wrapper {
        grid-row: 2;
        overflow-y: scroll;
        cursor: text;
    }

    .md-wrapper > .no-events-wrapper {
        pointer-events: none;
    }
</style>
