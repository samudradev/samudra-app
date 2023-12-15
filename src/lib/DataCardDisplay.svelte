<script lang="ts">
    import type { LemmaItem } from "../bindings/LemmaItem";
    import DisplayLemma from "../components/DisplayLemma.svelte";
    import ModalExportPng from "../components/ModalExportPng.svelte";

    export let data: LemmaItem;

    export let toggle_edit: VoidFunction;
    let export_png_modal;
    function open_export_modal() {
        export_png_modal.showModal();
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

<!-- TODO REFACTOR Put into main app to avoid rendering multiple export modal -->
<ModalExportPng bind:data bind:modal={export_png_modal} />
