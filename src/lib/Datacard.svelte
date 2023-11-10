<script lang="ts">
    import type { LemmaItem } from "../bindings/LemmaItem";

    import DataCardDisplay from "./DataCardDisplay.svelte";
    import DataCardEdit from "./DataCardEdit.svelte";

    let is_edit_mode: boolean = false;
    export let data: LemmaItem;

    function set_edit_mode_true() {
        console.log("Changing to edit mode!");
        is_edit_mode = true;
    }

    function set_as_data() {
        console.log("Changing display mode!");
        is_edit_mode = false;
    }
</script>

{#if !is_edit_mode}
    <DataCardDisplay {data} toggle_edit={set_edit_mode_true} />
{:else}
    <DataCardEdit {data} toggle_display={set_as_data} />
{/if}
