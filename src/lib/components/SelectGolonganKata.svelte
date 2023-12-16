<script lang="ts">
    // APIs
    import { onMount } from "svelte";
    // Components
    // Stores
    import GolonganKataStore from "../stores/GolonganKataStore";
    // Types

    // Initialize values
    export let golongan_kata: String;
    $: golongan_kata_list = [] as String[];

    onMount(() => {
        GolonganKataStore.subscribe((value) => {
            golongan_kata_list = value;
        });
    });
    // Event listeners
    // Callables
</script>

<select class="select" bind:value={golongan_kata} required>
    <option value="" disabled selected>Golongan kata</option>
    {#each golongan_kata_list as gk}
        <option value={golongan_kata}>{gk}</option>
    {/each}
</select>
