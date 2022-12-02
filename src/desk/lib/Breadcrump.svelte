<script lang="ts">
    import { invoke } from "@tauri-apps/api";

    export let path: string[] = [];

    function partDblClick(index: number) {
        const parts = path.slice(0, index + 1);
        const joined = parts.join("/");
        invoke("open", { url: joined });
    }
</script>

<span>
    {#each path as p, i}
        <span class="part" on:dblclick={(_) => partDblClick(i)}>{p}</span><i
            class="sep bi bi-caret-right-fill"
        />
    {/each}
</span>

<style>
    span {
        z-index: 2;
        display: flex;
        flex-direction: row;
        gap: 0.5rem;
        box-sizing: border-box;
        opacity: 0.6;
        user-select: none;
        -webkit-user-select: none;
    }

    .part {
        cursor: default;
        font-weight: 100;
    }

    .sep {
        font-size: 14px;
    }
</style>
