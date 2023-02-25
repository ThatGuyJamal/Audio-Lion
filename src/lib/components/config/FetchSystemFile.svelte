<script lang="ts">
	import {
		formatFileSize,
		extractFileNameWithoutExtension,
	} from "$lib/utils/format";
	let files: FileList;

	// Log the type of files
	$: console.log(files);
</script>

<div>
	<h1>Select a file path</h1>
	<p>This path will be used to play your music</p>
	<input type="file" bind:files />

	{#if files && files[0]}
		<p>
			Name: {extractFileNameWithoutExtension({
				fileName: files[0].name,
				capitalize: true,
			})}
			Size: {formatFileSize(files[0].size)}
		</p>
	{/if}
</div>
