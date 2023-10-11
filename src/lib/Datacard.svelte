<script lang="ts">
    import type { LemmaData } from "../bindings/LemmaData";

    import DataCardDisplay from "./DataCardDisplay.svelte";
    import DataCardEdit from "./DataCardEdit.svelte";

    let is_edit_mode: bool = false;
    export let data: LemmaData;

    function set_edit_mode_true() {
        console.log("Changing to edit mode!")
        is_edit_mode = true
    };

    function set_as_data() {
        console.log("Changing display mode!")
        is_edit_mode = false
    };
</script>

{#if !is_edit_mode }
    <DataCardDisplay data={data} toggle_edit={set_edit_mode_true}/>
    {:else}
    <DataCardEdit data={data} save_data={set_as_data}/>
{/if}