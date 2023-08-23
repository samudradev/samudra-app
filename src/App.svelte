<script lang="ts">
  import Footer from "./lib/Footer.svelte";
  import Greet from "./lib/Greet.svelte";
  import { open } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api";

  async function import_csv() {
    let selected = await open({
      directory: false,
      multiple: false,
      filters: [{ name: "CSV", extensions: ["csv"] }],
    });
    console.log(selected);
    // Refer the function in `main.rs` for the format of the csv
    let result = await invoke("import_from_csv", {
      path: selected,
    });
    console.log(result);
  }
</script>

<main class="container">
  <h1>Samudra</h1>
  <div class="row">
    <Greet />
  </div>
  <button on:click={import_csv}>Import from CSV</button>
  <Footer />
</main>
