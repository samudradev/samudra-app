<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { LemmaData } from "../bindings/LemmaData";
  import DataCard from "./DataCard.svelte";

  let name = "";
  let greetMsg: LemmaData[] = [];

  async function get_lemma() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("get", { lemma: name });
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={get_lemma}>
    <input id="greet-input" placeholder="Lemma..." bind:value={name} />
    <button type="submit">Get</button>
  </form>
  {#each greetMsg as msg}
    <DataCard data={msg} />
  {/each}
</div>
