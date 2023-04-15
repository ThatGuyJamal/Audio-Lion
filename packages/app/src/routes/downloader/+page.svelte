<script lang="ts">
	import { downloadAudioYt } from '$lib/bindings';
	import { tick } from 'svelte';

	$: downloadQueue = [] as string[];
	$: downloadUrl = '';

	async function download(event: Event) {
		const input = event.target as HTMLInputElement;

		downloadUrl = input.value;
		downloadQueue.push(downloadUrl);

		console.log(downloadUrl);
        console.table(downloadQueue);

        
		await downloadAudioYt(downloadUrl, 'Video').catch((err) => {
            console.error(err);
		});
        downloadUrl = '';

		await tick();
	}
</script>

<main>
	<div>
		{#each downloadQueue as url}
			<h2>Downloading: {url}</h2>
		{/each}
	</div>
	<label for="dir-input">Paste a folder path:</label>
	<input
		type="url"
		id="dir-input"
		class="file-input file-input-xs max-w-xs mt-4"
		on:input={download}
		bind:value={downloadUrl}
	/>
	<h2>Current url: {downloadUrl}</h2>
</main>
