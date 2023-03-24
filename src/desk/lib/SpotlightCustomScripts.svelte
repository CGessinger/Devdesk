<script lang="ts">
    import { getVault, readDirectory, isFile } from "../../utils/deskapi";
    import { formatter } from "../../utils/formatter";

    let project;
    let scripts = [];

    getVault().then(async (v) => {
        const scriptsDirectory = formatter.appendToPath(
            v.path,
            ".devdesk",
            "scripts"
        );
        const scriptPaths = await readDirectory(scriptsDirectory);

        for (const script of scriptPaths) {
            const ff = await isFile(script);
            if (!ff) continue;

            const scriptName = formatter.fileNameFrom(script);
            scripts = [...scripts, scriptName];
        }
    });

    window.addEventListener("spotlight_changed", (data: CustomEvent) => {
        project = data.detail;
    });

    function executeScriptByName(scriptName: string) {
        throw new Error("Not implemented yet");
    }
</script>

<div class="foot-wrapper">
    {#each scripts as script}
        <button title={script} on:click={(_) => executeScriptByName(script)}
            >{formatter.formatScriptName(script)}</button
        >
    {/each}
</div>
