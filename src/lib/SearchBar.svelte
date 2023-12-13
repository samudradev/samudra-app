<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import LemmaStore from "../Data";
  import type { LemmaItem } from "../bindings/LemmaItem";

  let lemma_query = "";

  // TODO FIX Function signature
  async function get_lemma() {
    let value: LemmaItem[] = await invoke("get", {
      lemma: lemma_query,
      konsep: "",
    });
    LemmaStore.set(value);
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={get_lemma}>
    <input id="greet-input" placeholder="Lemma..." bind:value={lemma_query} />
    <button type="submit">Get</button>
  </form>
</div>
