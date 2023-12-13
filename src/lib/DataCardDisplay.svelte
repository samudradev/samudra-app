<script lang="ts">
    import type { LemmaItem } from "../bindings/LemmaItem";
    import DisplayKonseps from "../components/DisplayKonseps.svelte";

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
        <DisplayKonseps konseps={data.konseps} />
    </div>
</div>
