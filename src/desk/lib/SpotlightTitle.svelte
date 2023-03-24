<script lang="ts">
    let title = "Spotlight";

    window.addEventListener("spotlight_changed", (data: CustomEvent) => {
        title = data.detail.name;
    });
</script>

<h1>{title}</h1>
