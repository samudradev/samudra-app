<script lang="ts">
  import Footer from "./lib/Footer.svelte";
  import SearchBar from "./lib/SearchBar.svelte";
  import { invoke } from "@tauri-apps/api";
  import { listen } from "@tauri-apps/api/event";

  import LemmaStore from "./Data.js";

  import DataCard from "./lib/Datacard.svelte";
  import DataCardNew from "./lib/DataCardNew.svelte";

  $: data = [];
  let database_name = "";
  let modal;

  const reload = () => {
    data = [];
  };
  LemmaStore.subscribe((value) => {
    data = value;
  });

  listen("register_database", (a) => {
    modal.showModal();
  });

  async function register_database() {
    console.log(database_name);
    if (database_name.trim().length != 0) {
      await invoke("register_database_and_set_active", { name: database_name });
      modal.close();
    }
  }

  // unlisten();
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
  <dialog bind:this={modal} class="modal">
    <div class="modal-box">
      <h3 class="font-bold text-lg">Register Database</h3>
      <p class="py-4">
        Give the database a name (if already available, will not create new but
        change that as active)
      </p>
      <form class="join">
        <input
          type="text"
          class="textarea join-item"
          bind:value={database_name}
        />
        <button on:click={register_database} class="join-item">Confirm</button>
      </form>
      <button on:click={() => modal.close()} class="btn-warning">Cancel</button>
    </div>
  </dialog>
  <hero class="hero min-h-200 bg-base-200 pt-10 pb-5">
    <div class="hero-content text-center">
      <div class="max-w-md">
        <h1 class="text-5xl font-bold pb-10">Samudra</h1>
        <SearchBar />
        <button on:click={reload}>Refresh</button>
      </div>
    </div>
  </hero>
  <div class="justify-center grid">
    {#if data.length == 0}
      <DataCardNew />
    {/if}
    {#each data as d}
      <DataCard data={d} />
    {/each}
  </div>
  <!--  <button on:click={import_csv}>Import from CSV</button>-->

  <Footer />
</main>
