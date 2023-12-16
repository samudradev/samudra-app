import { type Writable, writable } from "svelte/store";
import type { LemmaItem } from "../../bindings/LemmaItem";

const LemmaExportStore: Writable<LemmaItem> = writable();

export default LemmaExportStore;
