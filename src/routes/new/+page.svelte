<script>
	import { goto } from '$app/navigation';
</script>

<head>
	<link href="https://cdn.jsdelivr.net/npm/css.gg/icons/all.css" rel="stylesheet" />
</head>

<main class="container">
	<div class="hero">
		<div class="column">
			<h1 class="text-3xl font-bold py-10">New</h1>
		</div>
	</div>
	<button on:click={() => goto('/')}>Back</button>
</main>
