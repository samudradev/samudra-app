<script lang="ts">
    // APIs
    import { BaseDirectory, writeBinaryFile } from "@tauri-apps/api/fs";
    import { getName, getVersion } from "@tauri-apps/api/app";
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import { toPng } from "html-to-image";
    // Components
    import DisplayLemma from "./DisplayLemma.svelte";
    // Stores
    import LemmaExportStore from "../stores/LemmaExportStore";
    // Types

    // Initialize values
    $: data = $LemmaExportStore;
    let modal: HTMLDialogElement;
    let filepath: string = ".";
    let success: boolean = false;
    let display_name: string = "";
    let app_name: string = "";
    let app_version: string = "";

    onMount(async () => {
        display_name = await invoke("get_display_name");
        app_name = capitalize(await getName());
        app_version = await getVersion();
    });
    // Event listeners
    LemmaExportStore.subscribe((_) => {
        if (modal !== undefined && modal !== null) {
            modal.showModal();
        }
    });

    // Callables
    function capitalize(word: string) {
        return word[0].toUpperCase() + word.slice(1);
    }

    function convert_dataurl_to_binary(dataURI: string) {
        return Uint8Array.from(
            window.atob(dataURI.replace(/^data[^,]+,/, "")),
            (v) => v.charCodeAt(0),
        );
    }

    function export_png() {
        let node = document.getElementById(data.lemma);
        filepath = `./Samudra/export/${data.lemma}-${
            new Date().toISOString().split("T")[0]
        }.png `;
        toPng(node, {
            backgroundColor: "hsl(259 94% 51%)",
            height: node.scrollHeight + 10,
            width: node.scrollWidth + 10,
            pixelRatio: 5,
        }).then(async (data_url) => {
            writeBinaryFile(filepath, convert_dataurl_to_binary(data_url), {
                dir: BaseDirectory.Document,
            });
        });
        success = true;
    }
</script>

<dialog bind:this={modal} class="modal bg-primary">
    {#if data}
        <div class="relative w-[35em]">
            <div id={data.lemma} class="border-4 border-primary">
                <DisplayLemma {data} />
                <div
                    class="flex justify-between text-neutral-100 border-0 mx-4 -mt-2 mb-2"
                >
                    <span> @{display_name} </span>
                    <span class="badge badge-ghost">
                        {app_name} v{app_version}
                    </span>
                    <span> {new Date().toISOString().split("T")[0]} </span>
                </div>
            </div>

            <div
                class="menu menu-vertical absolute -right-[6em] top-0 py-[1em]"
            >
                <button
                    class="my-[0.2em]"
                    on:click={() => {
                        modal.close();
                        success = false;
                    }}
                >
                    Close
                </button>
                <button class="my-[0.2em]" on:click={export_png}>
                    Confirm
                </button>
            </div>
        </div>
    {/if}
    {#if success}
        <footer
            class="footer footer-center p-4 fixed inset-x-0 bottom-0 text-center"
        >
            <div role="alert" class="alert alert-success">
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
                <span>Image has been saved to $DOCUMENT{filepath.slice(1)}</span
                >
            </div>
        </footer>
    {/if}
</dialog>
