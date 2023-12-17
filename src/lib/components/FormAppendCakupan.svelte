<script lang="ts">
    // APIs
    // Components
    import DisplayCakupans from "./DisplayCakupans.svelte";
    // Stores
    // Types

    // Initialize values
    export let cakupans: string[];
    let cakupan_item: string;
    // Event listeners
    // Callables
    function append_new_cakupan() {
        cakupans.push(cakupan_item);
        cakupans = cakupans; // To force reload
        cakupan_item = "";
    }
</script>

<DisplayCakupans bind:cakupans editable={true} label={true} />
<form on:submit|preventDefault={append_new_cakupan} class="w-full">
    <span class="join w-full">
        <input
            type="text"
            placeholder="cakupan"
            class="textarea join-item w-full"
            bind:value={cakupan_item}
        />
        <button type="submit" class="join-item btn-primary">+</button>
    </span>
</form>
