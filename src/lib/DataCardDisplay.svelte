<script lang="ts">
    // APIs
    // Components
    import DisplayLemma from "./components/DisplayLemma.svelte";
    // Stores
    import LemmaExportStore from "./stores/LemmaExportStore";
    // Types
    import type { LemmaItem } from "../bindings/LemmaItem";

    // Initialize values
    export let data: LemmaItem;
    export let toggle_edit: VoidFunction;
    // Event listeners
    // Callable
    function open_export_modal() {
        LemmaExportStore.set(data);
    }
</script>

<div class="relative w-[35em]">
    <div class="inset-0 w-full">
        <DisplayLemma {data} />
    </div>

    <div class="menu menu-vertical absolute -right-[6em] top-0 py-[1em]">
        <button class="my-[0.2em]" on:click={toggle_edit}> Edit </button>
        <button class="my-[0.2em]" on:click={open_export_modal}>
            Export
        </button>
    </div>
</div>
