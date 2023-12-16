<script lang="ts">
    // APIs
    import { invoke } from "@tauri-apps/api/tauri";
    import { listen } from "@tauri-apps/api/event";
    // Components
    // Stores
    // Types

    // Initialize values
    let db_modal: HTMLDialogElement;
    let name_modal: HTMLDialogElement;
    let database_name: String = "";
    let display_name: String = "";
    // Event listeners
    listen("register_database", (a) => {
        db_modal.showModal();
    });
    listen("set_display_name", (a) => {
        name_modal.showModal();
    });
    // Callables
    async function register_database() {
        if (database_name.trim().length != 0) {
            await invoke("register_database_and_set_active", {
                name: database_name,
            });
            db_modal.close();
        }
    }
    async function set_display_name() {
        await invoke("set_display_name", { name: display_name });
        db_modal.close();
    }
</script>

<dialog bind:this={db_modal} class="modal">
    <div class="modal-box">
        <h3 class="font-bold text-lg">Register Database</h3>
        <p class="py-4">
            Give the database a name (if already available, will not create new
            but change that as active)
        </p>
        <form class="join">
            <input
                type="text"
                class="textarea join-item"
                bind:value={database_name}
            />
            <button on:click={register_database} class="join-item"
                >Confirm</button
            >
        </form>
        <button on:click={() => db_modal.close()} class="btn-warning"
            >Close</button
        >
    </div>
</dialog>

<dialog bind:this={name_modal} class="modal">
    <div class="modal-box">
        <h3 class="font-bold text-lg">Register Display Name</h3>
        <p class="py-4">Tell me your display name</p>
        <form class="join">
            <input
                type="text"
                class="textarea join-item"
                bind:value={display_name}
            />
            <button on:click={set_display_name} class="join-item"
                >Confirm</button
            >
        </form>
        <button on:click={() => name_modal.close()} class="btn-warning"
            >Close</button
        >
    </div>
</dialog>
