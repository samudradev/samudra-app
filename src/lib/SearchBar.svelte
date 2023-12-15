<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import LemmaStore from "../Data";
  import type { LemmaItem } from "../bindings/LemmaItem";

  export let reload: VoidFunction;
  let lemma = "";

  async function get_lemma() {
    let value: LemmaItem[] = await invoke("get_lemma", {
      lemma: lemma,
    });
    LemmaStore.set(value);
  }
</script>

<div class="row">
  <form class="join space-x-0" on:submit|preventDefault={get_lemma}>
    <input
      id="greet-input"
      class="join-item input"
      placeholder="Lemma..."
      bind:value={lemma}
    />
    <button class="btn btn-primary join-item" type="submit">Get</button>
  </form>
  <button class="btn btn-chost mx-2" on:click={reload}>Refresh</button>
</div>
