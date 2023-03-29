<script lang="ts">
	import { onMount, tick } from "svelte";
	import {
		extractFileName,
		getCurrentPlatform,
		getDirectoryPath,
		getFileExtension,
		getIndexByName,
	} from "$lib/utils/format";
	import DevInfo from "$lib/components/popups/dev-info.svelte";
	import { ApplicationConfigurationState } from "$lib/stores/ConfigStore";
	import { loadConfig, type ConfigResult } from "$lib/bindings";

	// the array of audio files to display to the user
	let audio_files_arr: string[] = [];

	// whether or not to display the audio player
	$: canDisplay = false;

	// load the current app config when the component is mounted
	onMount(async () => {
		const config = await loadConfig();

		// If the config exists, set the state and load the audio files
		ApplicationConfigurationState.set(config.data);

		const audioFiles = await loadAudioFiles(config);

		// If the audio files exist, store them in the array to display
		if (audioFiles) {
			audio_files_arr = audioFiles;
		}

		await tick();
	});

	async function loadAudioFiles(config: ConfigResult): Promise<string[] | null> {
		ApplicationConfigurationState.set(config.data);

		if (config.data.audio_directories.length <= 0) {
			canDisplay = false;
			return null;
		} else {
			canDisplay = true;
		}

		// implement

		return audio_files_arr;
	}
	async function play(path: string) {
		// implement
	}
</script>

<DevInfo />

<main>
	<h1 class="mb-8 text-2xl">Select a Song</h1>
	{#await $ApplicationConfigurationState}
		<p>fetching config file from system...</p>
	{:then}
		{#if canDisplay}
			{#if audio_files_arr}
				{#each audio_files_arr as file}
					<button
						class="btn btn-sm mb-5 text-teal-600"
						on:click={async () => await play(file)}
					>
						{extractFileName(file)}
					</button>
					<br />
				{/each}
				<div class="stats shadow mb-5 mt-3">
					<div class="stat">
						<div class="stat-title">Total Songs Loaded</div>
						<div class="stat-value">{audio_files_arr.length}</div>
						<!-- <div class="stat-desc"></div> -->
					</div>
				</div>
			{:else}
				<p class="text-red-600">No audio directories found</p>
			{/if}
		{:else}
			<div class="alert shadow-lg">
				<div>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						class="stroke-info flex-shrink-0 w-6 h-6"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
						/></svg
					>
					<div>
						<p class="font-bold text-red-600 ml-2 mr-2">Invalid Settings</p>
						<div class="text-xs">Missing Music Folders</div>
					</div>
				</div>
				<div class="flex-none">
					<button class="btn btn-sm"><a href="/config">Fix Settings</a></button>
				</div>
			</div>
		{/if}
	{:catch error}
		<p>{error.message}</p>
	{/await}
</main>
