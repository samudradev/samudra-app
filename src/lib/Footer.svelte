<script lang="ts">
    // APIs
    import { invoke } from "@tauri-apps/api/tauri";
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    // Components
    // Stores
    import LemmaStore from "./stores/LemmaStore";
    // Types
    import type { Counts } from "../bindings/Counts";

    // Initialize values
    $: database_url = "" as String;
    $: counts = {
        lemmas: 0,
        konseps: 0,
        golongan_katas: 0,
        cakupans: 0,
        kata_asings: 0,
    } as Counts;

    onMount(async () => {
        counts = await invoke("count_items", {});
        get_active_database_url();
    });
    // Event listeners
    LemmaStore.subscribe(async () => {
        counts = await invoke("count_items", {});
    });
    listen("active_database_changed", get_active_database_url);
    // Callables
    async function get_active_database_url() {
        database_url = await invoke("active_database_url");
    }
</script>

<footer
    class="footer footer-center p-3 bg-transparent text-xs fixed inset-x-0 bottom-0 text-center"
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
