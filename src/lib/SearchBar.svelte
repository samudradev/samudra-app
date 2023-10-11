<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { LemmaData } from "../bindings/LemmaData";
  import DataCard from "./DataCard.svelte";

  import LemmaStore from "../Data";

  let lemma_query = "";
  let konsep_query = "";

  async function insert_single_value() {
    await invoke("insert_single_value", {lemma: lemma_query, konsep: konsep_query})
  }

  async function get_lemma() {
    let value = await invoke("get", { lemma: lemma_query });
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    LemmaStore.update((prev) => {
    return value;
    })
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={get_lemma}>
    <input id="greet-input" placeholder="Lemma..." bind:value={lemma_query} />
    <button type="submit">Get</button>
  </form>
<!--  <form class="row" on:submit|preventDefault={insert_single_value}>-->
<!--    <input id="insert-lemma" placeholder="Lemma..." bind:value={lemma_query} />-->
<!--    <input id="insert-konsep" placeholder="Konsep..." bind:value={konsep_query} />-->
<!--    <button type="submit">Insert</button>-->
<!--  </form>-->
</div>
