<script lang="ts">
    import { onMount } from "svelte";
    import GolonganKataStore from "../GolonganKataStore";

    export let golongan_kata;
    $: golongan_kata_list = [] as String[];
    onMount(() => {
        GolonganKataStore.subscribe((value) => {
            golongan_kata_list = value;
        });
    });
</script>

<select class="select" bind:value={golongan_kata}>
    <option value="" disabled selected>Golongan kata</option>
    {#each golongan_kata_list as golongan_kata}
        <option value={golongan_kata}>{golongan_kata}</option>
    {/each}
</select>
