<script lang="ts">
  import Footer from "./lib/Footer.svelte";
  import SearchBar from "./lib/SearchBar.svelte";
  import { invoke } from "@tauri-apps/api";
  import { listen } from "@tauri-apps/api/event";

  import LemmaStore from "./Data";
  import GolonganKataStore from "./GolonganKataStore";

  import DataCard from "./lib/Datacard.svelte";
  import DataCardNew from "./lib/DataCardNew.svelte";
  import { onMount } from "svelte";

  $: data = [];
  $: golongan_kata = [];
  let database_name = "";
  let display_name = "";
  let db_modal;
  let name_modal;

  const reload = () => {
    data = [];
  };

  LemmaStore.subscribe((value) => {
    data = value;
  });

  onMount(async () => {
    golongan_kata = await invoke("get_golongan_kata_enumeration", {});
    GolonganKataStore.set(golongan_kata);
  });

  listen("register_database", (a) => {
    db_modal.showModal();
  });
  listen("set_display_name", (a) => {
    name_modal.showModal();
  });

  async function register_database() {
    if (database_name.trim().length != 0) {
      await invoke("register_database_and_set_active", { name: database_name });
      db_modal.close();
    }
  }
  async function set_display_name() {
    await invoke("set_display_name", { name: display_name });
    db_modal.close();
  }

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
    {/if}
    {#each data as d}
      <DataCard data={d} />
    {/each}
  </div>
  <!--  <button on:click={import_csv}>Import from CSV</button>-->
  <!-- To give space for footer -->
  <div class="min-h-[8em]"></div>
  <Footer />
</main>

<dialog bind:this={db_modal} class="modal">
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
    <button on:click={() => db_modal.close()} class="btn-warning">Close</button>
  </div>
</dialog>

<dialog bind:this={name_modal} class="modal">
  <div class="modal-box">
    <h3 class="font-bold text-lg">Register Display Name</h3>
    <p class="py-4">Tell me your display name</p>
    <form class="join">
      <input type="text" class="textarea join-item" bind:value={display_name} />
      <button on:click={set_display_name} class="join-item">Confirm</button>
    </form>
    <button on:click={() => name_modal.close()} class="btn-warning"
      >Close</button
    >
  </div>
</dialog>
