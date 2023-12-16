<script lang="ts">
  // APIs
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  // Components
  import Footer from "./lib/Footer.svelte";
  import SearchBar from "./lib/SearchBar.svelte";
  import DataCards from "./lib/DataCards.svelte";
  import DataCardNew from "./lib/DataCardNew.svelte";
  import ModalEventHandler from "./lib/components/ModalEventHandler.svelte";
  // Stores
  import LemmaStore from "./lib/stores/LemmaStore";
  import GolonganKataStore from "./lib/stores/GolonganKataStore";
  // Types
  import type { LemmaItem } from "./bindings/LemmaItem";
  import ModalExportPng from "./lib/components/ModalExportPng.svelte";

  // Initialize values
  $: data = [] as LemmaItem[];
  $: golongan_kata = [] as String[];

  onMount(async () => {
    golongan_kata = await invoke("get_golongan_kata_enumeration", {});
    GolonganKataStore.set(golongan_kata);
  });
  // Event listeners
  LemmaStore.subscribe((value) => {
    data = value;
  });
  // Callables
  const reload = () => {
    LemmaStore.update(() => {
      return [];
    });
  };

  // async function import_csv() {
  //   let selected = await open({
  //     directory: false,
  //     multiple: false,
  //     filters: [{ name: "CSV", extensions: ["csv"] }],
  //   });
  //   console.log(selected);
  //   // Refer the function in `main.rs` for the format of the csv
  //   let result = await invoke("import_from_csv", {
  //     path: selected,
  //   });
  //   console.log(result);
  // }
</script>

<main>
  <hero class="hero min-h-200 pt-10 pb-5">
    <div class="hero-content text-center">
      <div class="max-w-md">
        <h1 class="text-5xl font-bold pb-10">Samudra</h1>
        <SearchBar {reload} />
      </div>
    </div>
  </hero>
  <div class="justify-center grid">
    {#if data.length == 0}
      <DataCardNew />
    {:else}
      {#each data as d}
        <DataCards data={d} />
      {/each}
    {/if}
  </div>
  <!--  <button on:click={import_csv}>Import from CSV</button>-->
  <!-- To give space for footer -->
  <div class="min-h-[8em]"></div>
  <Footer />
</main>

<ModalEventHandler />
<ModalExportPng />
