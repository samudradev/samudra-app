<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import LemmaStore from "../Data";
  import type { LemmaItem } from "../bindings/LemmaItem";

  let lemma = "";

  async function get_lemma() {
    let value: LemmaItem[] = await invoke("get_lemma", {
      lemma: lemma,
    });
    LemmaStore.set(value);
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={get_lemma}>
    <input id="greet-input" placeholder="Lemma..." bind:value={lemma} />
    <button type="submit">Get</button>
  </form>
</div>
