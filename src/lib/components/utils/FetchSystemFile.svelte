<script lang="ts">
	let files: any;

	// Log the type of files
	$: console.log(`The type of the file prop is ${typeof files}`);
	$: console.log(files);
</script>

<div>
	<input type="file" bind:files />

	{#if files && files[0]}
		<p>
			{files[0].name}
		</p>
	{/if}
</div>
