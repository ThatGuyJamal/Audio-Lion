<script lang="ts">
	import type { PageData } from "./$types";
	export let result: PageData;
	console.log(result);
</script>

<main>
	<h1>Ok: {result.ok}</h1>
	<div>Message: {result.message}</div>
	<div>Data: {result.data}</div>
</main>
