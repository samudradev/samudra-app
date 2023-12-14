<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { listen } from "@tauri-apps/api/event";
    import type { Counts } from "../bindings/Counts";
    import LemmaStore from "../Data";

    $: database_url = "";
    $: counts = {} as Counts;
    onMount(async () => {
        counts = await invoke("count_items", {});
        get_active_database_url();
    });
    LemmaStore.subscribe(async () => {
        counts = await invoke("count_items", {});
    });
    async function get_active_database_url() {
        database_url = await invoke("active_database_url");
    }

    const listener = async () => {
        await listen("active_database_changed", get_active_database_url);
    };
    listener();
</script>

<!-- TODO MEDIUM PRIORITY Make footer not cover content -->
<footer
    class="footer footer-center p-3 bg-base-300 text-xs fixed bottom-0 left-0 text-center"
>
    <div class="join join-vertical gap-[0em]">
        <div class="stats join-item">
            <div class="stat">
                <span class="stat-title">lemma</span>
                <span class="stat-value text-base">{counts.lemmas}</span>
            </div>
            <div class="stat">
                <span class="stat-title">konsep</span>
                <span class="stat-value text-base">{counts.konseps}</span>
            </div>
            <div class="stat">
                <span class="stat-title">golongan kata</span>
                <span class="stat-value text-base">{counts.golongan_katas}</span
                >
            </div>
            <div class="stat">
                <span class="stat-title">cakupan</span>
                <span class="stat-value text-base">{counts.cakupans}</span>
            </div>
            <div class="stat">
                <span class="stat-title">kata asing</span>
                <span class="stat-value text-base">{counts.kata_asings}</span>
            </div>
        </div>
        <div class="join-item card bg-neutral-100 p-2">{database_url}</div>
    </div>
</footer>
