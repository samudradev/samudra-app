// Script structure:
// APIs
// Components
// Stores
// Types
// Initialize values
// Event listeners
// Callables

import { type Writable, writable } from "svelte/store";

const GolonganKataStore: Writable<String[]> = writable([]);

export default GolonganKataStore;
