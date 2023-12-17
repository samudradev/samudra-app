<script lang="ts">
    // APIs
    // Components
    import DisplayKataAsings from "./DisplayKataAsings.svelte";
    // Stores
    // Types
    import type { KataAsingItem } from "../../bindings/KataAsingItem";

    // Initialize values
    export let kata_asings: KataAsingItem[];
    let kata_asing_item: KataAsingItem = { nama: "", bahasa: "" };
    // Event listeners
    // Callables
    function append_new_kata_asing() {
        kata_asings.push(kata_asing_item);
        kata_asings = kata_asings; // To force reload
        kata_asing_item = { nama: "", bahasa: "" };
    }
</script>

<DisplayKataAsings bind:kata_asings editable={true} label={true} />
<form
    on:submit|preventDefault={append_new_kata_asing}
    class="form-control w-full"
>
    <span class="join w-full">
        <input
            type="text"
            placeholder="kata"
            class="textarea join-item w-1/2"
            bind:value={kata_asing_item.nama}
        />
        <input
            type="text"
            placeholder="bahasa"
            class="textarea join-item w-1/2"
            bind:value={kata_asing_item.bahasa}
        />
        <button type="submit" class="join-item btn-primary">+</button>
    </span>
</form>
