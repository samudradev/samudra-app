<script lang="ts">
    // APIs
    // Components
    import DataCardDisplay from "./DataCardDisplay.svelte";
    import DataCardEdit from "./DataCardEdit.svelte";
    // Stores
    // Types
    import type { LemmaItem } from "../bindings/LemmaItem";

    // Initialize values
    export let data: LemmaItem;
    let is_edit_mode: boolean = false;
    // Event listeners
    // Callable
</script>

{#if !is_edit_mode}
    <DataCardDisplay {data} toggle_edit={() => (is_edit_mode = true)} />
{:else}
    <DataCardEdit {data} toggle_display={() => (is_edit_mode = false)} />
{/if}
