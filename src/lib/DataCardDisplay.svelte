<script lang="ts">
    import type { LemmaItem } from "../bindings/LemmaItem";
    import DisplayKonseps from "../components/DisplayKonseps.svelte";
    import { toPng } from "html-to-image";

    import { BaseDirectory, writeBinaryFile } from "@tauri-apps/api/fs";
    import DisplayLemma from "../components/DisplayLemma.svelte";

    export let data: LemmaItem;

    export let toggle_edit;
    let alert;
    let filepath: string = ".";

    const convertDataURIToBinary = (dataURI: string) =>
        Uint8Array.from(window.atob(dataURI.replace(/^data[^,]+,/, "")), (v) =>
            v.charCodeAt(0),
        );

    function export_png() {
        alert.showModal();
        let node = document.getElementById(data.lemma);
        filepath = `./Samudra/export/${data.lemma}-${
            new Date().toISOString().split("T")[0]
        }.png`;
        toPng(node).then(async (data_url) => {
            writeBinaryFile(filepath, convertDataURIToBinary(data_url), {
                dir: BaseDirectory.Document,
            });
        });
    }
</script>

<div class="relative w-[30em]" id={data.lemma}>
    <div class="inset-0 w-full">
        <DisplayLemma {data} />
    </div>

    <div class="menu menu-vertical absolute -right-[6em] top-0 py-[1em]">
        <button class="my-[0.2em]" on:click={toggle_edit}> Edit </button>
        <button class="my-[0.2em]" on:click={export_png}> Export </button>
    </div>
</div>

<dialog class="modal fixed bottom-0" bind:this={alert}>
    <div role="alert" class="modal-box bg-success flex space-between space-x-2">
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class="stroke-current shrink-0 h-6 w-6"
            fill="none"
            viewBox="0 0 24 24"
            ><path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
            /></svg
        >
        <span>Image exported to $DOCUMENT{filepath.substring(1)}</span>
    </div>
    <form method="dialog" class="modal-backdrop transparent">
        <button
            on:click={() => {
                alert.close();
            }}
        ></button>
    </form>
</dialog>
