import {type Writable, writable} from "svelte/store";
import type {LemmaData} from "./bindings/LemmaData";
import DataCard from "./lib/Datacard.svelte";

const LemmaStore: Writable<LemmaData[]> = writable([]);

export default LemmaStore;
