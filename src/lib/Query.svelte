<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import Datacard from './Datacard.svelte';
	import type { SamudraData } from '../dataclasses/SamudraData';

	let name: string | null = null;
	let konsep: string | null = null;
	let data: SamudraData[] = [];

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		data = await invoke('get', { lemma: name });
	}
</script>

<div class="space-y-10">
	<form class="row space-x-5" on:submit|preventDefault={greet}>
		<input
			bind:value={name}
			id="greet-input"
			placeholder="Lemma"
			class="input input-primary w-full max-w-xs"
		/>
		<!-- <input
			bind:value={konsep}
			id="greet-input"
			placeholder="Konsep"
			class="input input-primary w-full max-w-xs"
		/> -->
		<button type="submit" class="btn btn-primary"><i class="gg-search" /></button>
	</form>
	{#each data as d}
		<Datacard data={d} single={false} />
	{/each}
</div>

<style>
	.input {
		border: 0px;
	}
</style>
