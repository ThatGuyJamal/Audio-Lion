<script lang="ts">
	import { onMount, tick } from "svelte";
	import { writable } from "svelte/store";
	import { getAudioFiles, loadAppConfig, playAudioFile } from "$lib/utils/tauri";
	import { AudioFileType, type AppConfig } from "$lib/types/AppConfig";
	import {
		extractFileName,
		getDirectoryPath,
		getFileExtension,
		getIndexByName,
	} from "$lib/utils/format";
	import DevInfo from "$lib/components/popups/dev-info.svelte";

	// The updated state of the component
	const viewState = writable<AppConfig | null>();
	let audio_files_arr: string[] = [];
	$: canDisplay = false; // whether or not to display the audio player

	loadAppConfig().then(async (data) => {
		let result = (await loadAudioFiles(data).catch(() => [])) as string[];
		audio_files_arr = result;
		// console.log("audio_files_arr", audio_files_arr);
	});

	// load the current app config when the component is mounted
	onMount(async () => {
		const load = await loadAppConfig();
	});

	async function loadAudioFiles(data: AppConfig | null) {
		if (data) {
			viewState.set(data);

			if (data.audio_directories.length <= 0) {
				canDisplay = false;
				return;
			} else {
				canDisplay = true;
			}

			let shouldLoadMp3 = data.audio_file_types_allowed.find(
				(type) => type === "mp3"
			)
				? "yes"
				: "no";
			let shouldLoadWav = data.audio_file_types_allowed.find(
				(type) => type === "wav"
			)
				? "yes"
				: "no";

			if (shouldLoadMp3 === "yes") {
				const mp3Files = await getAudioFiles(AudioFileType.MP3);

				// console.log("MP3", mp3Files);

				for (let i = 0; i < mp3Files.length; i++) {
					audio_files_arr.push(mp3Files[i]);
				}
			}

			if (shouldLoadWav === "yes") {
				const wavFiles = await getAudioFiles(AudioFileType.WAV);

				// console.log("WAV", wavFiles);

				for (let i = 0; i < wavFiles.length; i++) {
					audio_files_arr.push(wavFiles[i]);
				}
			}

			return audio_files_arr;
		}

		viewState.set(null);
		await tick();
		return [];
	}

	async function play(path: string) {
		let fileName = extractFileName(path);
		let fileExtension = getFileExtension(path);
		let filePath = getDirectoryPath(path, "windows");
		let fileIndex = getIndexByName(path, audio_files_arr);

		if (fileExtension === "mp3") {
			await playAudioFile({
				file_path: filePath,
				file_type: AudioFileType.MP3,
				file_index: fileIndex,
			});
		}

		if (fileExtension === "wav") {
			await playAudioFile({
				file_path: filePath,
				file_type: AudioFileType.WAV,
				file_index: fileIndex,
			});
		}
	}
</script>

<DevInfo />

<main>
	<h1 class="mb-8 text-2xl">Select a Song</h1>
	{#await $viewState}
		<p>fetching config file from system...</p>
	{:then}
		{#if canDisplay}
			{#if audio_files_arr}
				{#each audio_files_arr as file}
					<button class="btn btn-sm mb-5" on:click={async () => await play(file)}>
						{extractFileName(file)}
					</button>
					<br />
				{/each}
				<div class="stats shadow">
					<div class="stat">
						<div class="stat-title">Total Songs Loaded</div>
						<div class="stat-value">{audio_files_arr.length}</div>
						<!-- <div class="stat-desc"></div> -->
					</div>
				</div>
			{:else}
				<p>No audio directories found</p>
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
						<h3 class="font-bold">Invalid Settings</h3>
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
