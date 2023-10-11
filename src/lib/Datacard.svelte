<script lang="ts">
    import type { LemmaData } from "../bindings/LemmaData";

    export let data: LemmaData;

    function edit_this() {
        console.log("Hello!")
    //     TODO Go to edit mode
    };
</script>

<div class="card card-normal m-4 w-96 bg-base-100 shadow-xl">
    <div class="card-body">
        <div class="grid w-full grid-flow-row-dense grid-cols-4 grid-rows-1">

        <h2 class="card-title col-span-3">{data.nama}</h2>
        <button class="btn btn-ghost btn-xs col-span-1" on:click={edit_this}>
            Edit
        </button>
        </div>
        {#if data.konseps != null}
            {#each data.konseps as konsep, i}
                <div class="text-left">
                    {i + 1}.
                    <span class="badge">{konsep.golongan_kata.id}</span>
                    {konsep.keterangan}
                    {#if konsep.cakupan != null}
                        <div class="column">
                            {#each konsep.cakupan as cakupan}
                                <div class="badge">{cakupan.nama}</div>
                            {/each}
                        </div>
                    {/if}
                    {#if konsep.kata_asing != null}
                        <div class="column">
                            {#each konsep.kata_asing as kata_asing}
                                <div class="badge">{kata_asing.nama}</div>
                            {/each}
                        </div>
                    {/if}
                </div>
            {/each}
        {/if}
    </div>
</div>
