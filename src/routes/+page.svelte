<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	import Query from '../lib/Query.svelte';

	$: database_url = '';

	onMount(async () => {
		database_url = await invoke('active_database_url');
	});
</script>

<head>
	<link href="https://cdn.jsdelivr.net/npm/css.gg/icons/all.css" rel="stylesheet" />
</head>

<main class="container">
	<div class="hero">
		<div class="column">
			<h1 class="text-3xl font-bold py-10">Samudra</h1>
			<Query />
		</div>
	</div>
	<footer class="footer footer-center p-3 bg-base-300">
		<div class="row">
			<i class="gg-link" />
			<p>{database_url}</p>
		</div>
	</footer>
</main>

<style>
	.footer {
		position: fixed;
		left: 0;
		bottom: 0;
		width: 100%;
		text-align: center;
	}
</style>
