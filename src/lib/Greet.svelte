<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { LemmaData } from "../bindings/LemmaData";
  import DataCard from "./DataCard.svelte";

  let lemma = "";
  let konsep = "";
  let greetMsg: LemmaData[] = [];

  async function insert_single_value() {
    await invoke("insert_single_value", {lemma: lemma, konsep: konsep})
  }

  async function get_lemma() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("get", { lemma: lemma });
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={get_lemma}>
    <input id="greet-input" placeholder="Lemma..." bind:value={lemma} />
    <button type="submit">Get</button>
  </form>
  <form class="row" on:submit|preventDefault={insert_single_value}>
    <input id="insert-lemma" placeholder="Lemma..." bind:value={lemma} />
    <input id="insert-konsep" placeholder="Konsep..." bind:value={konsep} />
    <button type="submit">Insert</button>
  </form>
  {#each greetMsg as msg}
    <DataCard data={msg} />
  {/each}
</div>
