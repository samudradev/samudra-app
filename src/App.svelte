<script lang="ts">
  import Footer from "./lib/Footer.svelte";
  import SearchBar from "./lib/SearchBar.svelte";
  import { open } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api";

  import LemmaStore from "./Data.js";
  import type { LemmaItem } from "./bindings/LemmaItem";

  import DataCard from "./lib/Datacard.svelte";
  import DataCardNew from "./lib/DataCardNew.svelte";

  let data: LemmaItem[] = [];

  LemmaStore.subscribe((value) => {
    data = value;
  });

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
  <hero class="hero min-h-200 bg-base-200 pt-10 pb-5">
    <div class="hero-content text-center">
      <div class="max-w-md">
        <h1 class="text-5xl font-bold pb-10">Samudra</h1>
        <SearchBar />
        <!-- TODO Refresh button -->
      </div>
    </div>
  </hero>
  {#if data.length == 0}
    <DataCardNew />
  {/if}
  <div class="justify-center grid">
    {#each data as d}
      <DataCard data={d} />
    {/each}
  </div>
  <!--  <button on:click={import_csv}>Import from CSV</button>-->
  <Footer />
</main>
