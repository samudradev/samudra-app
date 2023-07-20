<script lang="ts">
import type { SamudraData } from 'src/dataclasses/SamudraData';

	export let data: SamudraData;

	export let single: boolean;

	console.log(data);
</script>

<div class="card card-normal m-100 w-96 bg-base-100 shadow-xl">
	<div class="card-body">
		{#if single}
			<h2 class="card-title">{data ? data.lemma.nama : data}</h2>
		{:else}
			<a class="card-title" href="/data/{data.lemma.nama}">{data ? data.lemma.nama : data}</a>
		{/if}
		{#each data.konseps as konsep, i}
			<div class="text-left">
				{i + 1}.
				<span class="badge">{konsep.golongan_id}</span>
				{konsep.keterangan}
				<!-- <div class="column">
					{#each konsep.cakupans as cakupan}
						<div class="badge">{cakupan.nama}</div>
					{/each}
				</div>
				<div class="column">
					{#each konsep.kata_asings as kata_asing}
						<div class="badge">{kata_asing.nama}</div>
					{/each}
				</div> -->
			</div>
		{/each}
	</div>
</div>
