<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { listen } from "@tauri-apps/api/event";

    $: database_url = "";

    async function get_active_database_url() {
        database_url = await invoke("active_database_url");
    }

    onMount(() => {
        get_active_database_url();
    });
    const listener = async () => {
        await listen("active_database_changed", get_active_database_url);
    };
    listener();
</script>

<main>
    <footer class="footer footer-center p-3 bg-base-300">
        <p>{database_url}</p>
    </footer>
</main>

<style>
    .footer {
        position: fixed;
        left: 0;
        bottom: 0;
        width: 100%;
        text-align: center;
    }
</style>
