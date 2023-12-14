<script lang="ts">
    import DisplayCakupans from "./DisplayCakupans.svelte";

    export let cakupans: string[];
    let cakupan_item: string;
    function append_new_cakupan() {
        cakupans.push(cakupan_item);
        cakupans = cakupans; // To force reload
        cakupan_item = "";
    }
</script>

<form on:submit|preventDefault={append_new_cakupan} class="w-full">
    <div class="label">
        <span class="label-text-alt">Cakupan</span>
    </div>
    <DisplayCakupans bind:cakupans />
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
