<script lang="ts">
	import Query from '../lib/Query.svelte';
	import { goto } from '$app/navigation';
</script>

<head>
	<link href="https://cdn.jsdelivr.net/npm/css.gg/icons/all.css" rel="stylesheet" />
</head>

<main class="container">
	<div class="hero">
		<div class="column">
			<h1 class="text-3xl font-bold py-10">Samudra</h1>
			<Query />
			<button on:click={() => goto('/new')}>New</button>
		</div>
	</div>
</main>
