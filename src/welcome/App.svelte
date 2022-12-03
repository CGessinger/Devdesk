<script>
    import { invoke } from "@tauri-apps/api";

    import { open } from "@tauri-apps/api/dialog";
    import { documentDir } from "@tauri-apps/api/path";
    import { appWindow } from "@tauri-apps/api/window";

    import Logo from "./lib/Logo.svelte";

    async function selectVault() {
        const selected = await open({
            multiple: false,
            directory: true,
            defaultPath: await documentDir(),
        });

        if (!selected) return;
        console.log("set");
        invoke("set_vault_path", { path: selected });
    }

    appWindow.listen("vault_set_success", () => {
        console.log("success");
        window.location.href = "/desk.html";
    });

    setTimeout(() => {
        let hello = document.querySelector(".hello");
        hello.classList.add("active");
        setTimeout(() => {
            let d = document.querySelector(".vault div");
            let b = document.querySelector(".vault .button-wrapper");
            d.classList.add("active");
            b.classList.add("active");
        }, 2.5 * 1000);
    }, 1.8 * 1000);
</script>

<main>
    <Logo />
    <div class="hello">Hello</div>
    <div class="vault">
        <div>Please Select Your Programming Folder</div>
        <div class="button-wrapper">
            <button type="button" on:click={selectVault}>Select</button>
        </div>
    </div>
</main>

<style global>
    main {
        margin: 0;
        padding: 0;
        height: 100vh;
        width: 100vw;
        background: rgb(28, 28, 30);
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        grid-template-rows: auto auto 1fr;
    }

    .hello {
        grid-column: 2;
        grid-row: 2;
        text-align: center;
        font-size: 5rem;
        transform: translateY(-200%);
        transition-timing-function: ease-in;
        visibility: hidden;
    }

    .hello.active {
        visibility: visible;
        transform: translateY(0);
        transition-timing-function: ease-in;
        transition: 2.5s;
    }

    .vault {
        grid-column: 2;
        grid-row: 3;
        font-size: 2rem;
        display: flex;
        flex-direction: column;
        justify-content: center;
        gap: 1em;
    }

    .vault div {
        text-align: center;

        transform: translateY(-100%);
        transition-timing-function: ease-in;
        visibility: hidden;
    }

    .vault div.active {
        visibility: visible;
        transition: ease-in;
        transform: translateY(0);
        transition-timing-function: ease-in;
        transition: 1.5s;
    }

    .vault .button-wrapper {
        transform: translateY(-100%);
        transition-timing-function: ease-in;
        visibility: hidden;
    }

    .vault .button-wrapper.active {
        visibility: visible;
        transition: ease-in;
        transform: translateY(0);
        transition-timing-function: ease-in;
        transition: 1.5s;
    }

    .vault button {
        width: 100%;
    }
</style>
