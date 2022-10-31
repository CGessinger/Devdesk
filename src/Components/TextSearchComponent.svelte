<script lang="ts">
    import { emit } from "@tauri-apps/api/event";

    function searchChange(e) {
        const value: String = e.target.value;
        emit('searchInputChange', value);
    }
</script>

<div class="wrapper">
    <i class="bi bi-search search-icon"></i>
    <input class="search form-control" type="text" on:input={searchChange} placeholder="Search"/>
</div>

<style>
    .wrapper {
        width: 100%;
        position: relative;
    }

    .search-icon {
        color: white;
        position: absolute;
        top: 50%;
        transform: translateY(-50%);
        left: 10px;
    }

    .search {
        display: block;
        color: #fff;
        background-color: rgba(255, 255, 255, 0.1);
        border: 1px solid rgba(255, 255, 255, 0.3);
        border-radius: 0.5rem;
        padding-left: 2.5rem;
    }

    .search:before {
        content: "Search";
    }

    .search::placeholder {
        font-weight: 300;
        color: #fff;
        opacity: 0.5;
    }
</style>