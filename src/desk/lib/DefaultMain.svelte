<script lang="ts">
    import ProjectList from "./ProjectList.svelte";
    export let recent = [];
</script>

<div class="wrapper">
    <div class="content">
        <button class="action new">
            <i class="bi bi-plus-square" />
            <span>New Project</span>
        </button>
        <button class="action about">
            <i class="bi bi-info-square" />
            <span>About</span>
        </button>

        <button class="action settings">
            <i class="bi bi-toggles" />
            <span>Settings</span>
        </button>
        <div class="action recent">
            <span>Recent <i class="bi bi-stars" /></span>
            <div class="recent-list">
                <ProjectList projects={recent} />
            </div>
        </div>
    </div>
</div>

<style>
    .wrapper {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 100%;
    }

    .content {
        width: 512px;
        height: 512px;
        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: repeat(4, 1fr);
    }

    .content > .action:hover {
        background-color: rgb(255, 45, 85);
    }

    .content > .action {
        height: calc(100% - 1rem);
        margin: 0.5rem;
        background: rgb(44, 44, 46);
        border-radius: 0.325rem;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 10px;
    }

    .content span {
        font-weight: bolder;
        font-size: 1.25rem;
    }

    .content i {
        font-size: 30px;
    }

    .new {
        grid-column: 1;
        grid-row: 1 / span 3;
    }

    .about {
        grid-row: 4;
        grid-column: 1;
    }

    .settings {
        grid-column: 2;
        grid-row: 1;
    }

    .recent {
        grid-column: 2;
        grid-row: 2 / span 3;
        align-items: flex-start !important;
        justify-content: start !important;
        padding: 1rem;
    }

    .recent:hover {
        background: rgb(44, 44, 46) !important;
    }

    .recent > span > i {
        font-size: unset;
    }

    .recent-list {
        height: 100%;
        overflow-y: scroll;
    }
</style>
