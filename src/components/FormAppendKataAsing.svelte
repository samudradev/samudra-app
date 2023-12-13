<script lang="ts">
    import type { KataAsingItem } from "../bindings/KataAsingItem";

    export let kata_asings: KataAsingItem[];
    let kata_asing_item: KataAsingItem = { nama: "", bahasa: "" };

    function append_new_kata_asing() {
        kata_asings.push(kata_asing_item);
        kata_asings = kata_asings; // To force reload
        kata_asing_item = { nama: "", bahasa: "" };
    }
</script>

<form
    on:submit|preventDefault={append_new_kata_asing}
    class="form-control w-full"
>
    <div class="label">
        <span class="label-text-alt">Kata asing</span>
    </div>
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
        <button type="submit" class="join-item">+</button>
    </span>
    {#each kata_asings as kata_asing}
        <div>{kata_asing.nama}: {kata_asing.bahasa}</div>
    {/each}
</form>
