<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import LemmaStore from "../Data";

  let lemma_query = "";
  let konsep_query = "";

  async function insert_single_value() {
    await invoke("insert_single_value", {
      lemma: lemma_query,
      konsep: konsep_query,
    });
  }

  async function get_lemma() {
    let value = await invoke("get", { lemma: lemma_query, konsep: "" });
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    LemmaStore.update((prev) => {
      return value;
    });
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={get_lemma}>
    <input id="greet-input" placeholder="Lemma..." bind:value={lemma_query} />
    <button type="submit">Get</button>
  </form>
</div>
