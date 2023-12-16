import { type Writable, writable } from "svelte/store";
import type { LemmaItem } from "../../bindings/LemmaItem";

const LemmaStore: Writable<LemmaItem[]> = writable([]);

export default LemmaStore;
