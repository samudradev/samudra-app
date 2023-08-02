<script lang="ts">
    import type { LemmaData } from "../bindings/LemmaData";

    export let data: LemmaData;
</script>

<div class="card card-normal m-100 w-96 bg-base-100 shadow-xl">
    <div class="card-body">
        <h2 class="card-title">{data.nama}</h2>
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
