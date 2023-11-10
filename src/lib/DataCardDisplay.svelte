<script lang="ts">
    import type { LemmaItem } from "../bindings/LemmaItem";

    export let data: LemmaItem;

    export let toggle_edit: CallableFunction;
</script>

<div class="card card-normal m-4 w-96 bg-base-100 shadow-xl">
    <div class="card-body">
        <div class="grid w-full grid-flow-row-dense grid-cols-4 grid-rows-1">
            <h2 class="card-title col-span-3">{data.lemma}</h2>
            <button
                class="btn btn-ghost btn-xs col-span-1"
                on:click={toggle_edit}
            >
                Edit
            </button>
        </div>
        {#if data.konseps != null}
            {#each data.konseps as konsep, i}
                <div class="text-left">
                    {i + 1}.
                    <span class="badge">{konsep.golongan_kata}</span>
                    {konsep.keterangan}
                    {#if konsep.cakupans != null}
                        <div class="column">
                            {#each konsep.cakupans as cakupan}
                                <div class="badge">{cakupan}</div>
                            {/each}
                        </div>
                    {/if}
                    {#if konsep.kata_asing != null}
                        <div class="column">
                            {#each konsep.kata_asing as kata_asing}
                                <div class="badge">
                                    {kata_asing.nama} ({kata_asing.bahasa})
                                </div>
                            {/each}
                        </div>
                    {/if}
                </div>
            {/each}
        {/if}
    </div>
</div>
