<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { appWindow } from "@tauri-apps/api/window";

    let backupRunning = false;
    let backupStatusText = "";
    let backupSize = 0;

    function backup() {
        invoke("backup_vault");
        backupRunning = true;

        appWindow.listen("vault_backup_start", (_) => {
            backupStatusText = "calculating size...";
        });

        appWindow.listen("vault_size", (event) => {
            const size = Math.round((event.payload as number) * 100) / 100;
            backupSize = size;
            backupStatusText = `backing up ${size}GB ...`;
        });

        appWindow.listen("vault_backup_progress", (event) => {
            const size = Math.round((event.payload as number) * 100) / 100;
            const progress = Math.round((size / backupSize) * 100 * 100) / 100;
            backupStatusText = `${progress}%`;
        });

        appWindow.listen("vault_backup_end", (_) => {
            backupRunning = false;
            backupStatusText = "";
        });
    }
</script>

<div>
    <button
        id="backup"
        title="Backup Vault"
        on:click={backup}
        disabled={backupRunning}
    >
        <i class="bi bi-file-earmark-zip-fill" />
        <span>{backupStatusText}</span>
    </button>
</div>

<style>
    div {
        display: flex;
        flex-direction: row;
        height: 100%;
    }

    #backup {
        margin-left: auto;
        font-size: 1rem;
        padding: 0.45rem;
        background: none;
    }

    button:hover {
        background: rgba(44, 44, 46, 0.5) !important;
    }
</style>
