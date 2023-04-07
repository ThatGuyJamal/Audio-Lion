<script lang="ts">
	import { loadConfig } from '$lib/bindings';
	import { ApplicationConfigurationState, type AudioState } from '$lib/store';
	import { extractFileName, loadAudioFiles } from '$lib/utils';
	import { onMount, tick } from 'svelte';
	import AudioFiles from '../../components/audio-files.svelte';
	import DevInfo from '../../components/popups/dev-info.svelte';
	import { AudioAPI } from '$lib/Audio';

	let files: AudioState[] = [];

	// whether or not to display the audio player
	$: display = false;

	// load the current app config when the component is mounted
	onMount(async () => {
		const config = await loadConfig();

		// If the config exists, set the state and load the audio files
		ApplicationConfigurationState.set(config.data);

		const audioFiles = await loadAudioFiles(config);

		// If the audio files exist, store them in the array to display
		if (audioFiles) {
			for (const file of audioFiles) {
				let _preload = true;
				if (audioFiles.length > 100) _preload = false;

				// Create a new Howl instance for each file
				const _api = new AudioAPI();

				let getName = extractFileName(file);

				files.push({
					name: getName,
					file: file,
					api: _api
				});
			}

			display = true;
			console.log('files', files);
		}

		await tick();
	});
</script>

<DevInfo />

<main>
	<h1 class="mb-8 text-2xl">Select a Song</h1>
	{#await $ApplicationConfigurationState}
		<p>fetching config file from system...</p>
	{:then}
		<AudioFiles audio_files={files} canDisplay={display} />
	{:catch error}
		<p>{error.message}</p>
	{/await}
</main>
