<div class="wrapper">
    <i class="bi bi-search search-icon" />
    <input class="search form-control" type="text" placeholder="Suchen" />
</div>

<style>
    .wrapper {
        display: inline-block;
        width: 100%;
        position: relative;
        height: 2rem;
    }

    .search-icon::before {
        color: white;
        position: absolute;
        top: 50%;
        transform: translateY(-60%);
        left: 10px;
        scale: 0.8;
    }

    .search {
        display: block;
        width: 100%;
        color: #fff;
        background-color: rgba(255, 255, 255, 0.1);
        border: 1px solid rgba(255, 255, 255, 0.3);
        padding-left: 2rem;
        margin: 0;
        box-sizing: border-box;
        height: 100%;
    }
</style>
