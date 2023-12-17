<script lang="ts">
    // APIs
    // Components
    // Stores
    // Types

    // Initialize values
    export let lemma: string;
    export let submit: VoidFunction;
    export let toggle: VoidFunction = () => {};
    // Event listeners
    // Callables
</script>

<form
    class="join"
    on:submit|preventDefault={submit}
    on:submit|preventDefault={toggle}
>
    <input
        type="text"
        bind:value={lemma}
        class="input input-bordered w-full join-item"
        required
    />
    <button type="submit" class="btn-primary join-item"> Save </button>
</form>
