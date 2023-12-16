<script>
    // APIs
    // Components
    import DisplayKonseps from "./DisplayKonseps.svelte";
    // Stores
    // Types

    // Initialize values
    export let data;
    // Event listeners
    // Callables
</script>

<div class="card card-normal m-4 bg-base-100 shadow-xl">
    <div class="card-body">
        <div class="grid w-full grid-flow-row-dense grid-cols-4 grid-rows-1">
            <h2 class="card-title col-span-3">{data.lemma}</h2>
        </div>
        <DisplayKonseps konseps={data.konseps} />
    </div>
</div>
