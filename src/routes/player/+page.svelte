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

	// The updated state of the component
	const viewState = writable<AppConfig | null>();
	let audio_files_arr: string[] = [];
	$: canDisplay = false; // whether or not to display the audio player

	loadAppConfig().then(async (data) => {
		let result = (await loadAudioFiles(data).catch(() => [])) as string[];
		audio_files_arr = result;
		console.log("audio_files_arr", audio_files_arr);
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

		console.log("index:", fileIndex)

		console.log("filePath", filePath)

		if (fileExtension === "mp3") {
			await playAudioFile({
				file_path: filePath,
				file_type: AudioFileType.MP3,
				file_index: fileIndex,
			})
				.then((res) => {
					if (res) {
						console.log(`Playing ${fileName}...`);
					} else {
						console.log(`Failed to play ${fileName}`);
					}
				})
				.catch((err) => {
					console.log(err);
				});
		}

		if (fileExtension === "wav") {
			await playAudioFile({
				file_path: filePath,
				file_type: AudioFileType.WAV,
				file_index: fileIndex,
			})
				.then((res) => {
					if (res) {
						console.log(`Playing ${fileName}...`);
					} else {
						console.log(`Failed to play ${fileName}`);
					}
				})
				.catch((err) => {
					console.log(err);
				});
		}
	}
</script>

<div>
	<h1>Audio Player</h1>
	{#await $viewState}
		<p>fetching config file from system...</p>
	{:then}
		{#if canDisplay}
			{#if audio_files_arr}
				{#each audio_files_arr as file}
					<button
						on:click={() => {
							console.log("clicked", file);
							play(file);
						}}
					>
						{extractFileName(file)}
					</button>
					<br />
					<br />
				{/each}
			{:else}
				<p>No audio directories found</p>
			{/if}
		{:else}
			<p>
				No audio folder config found in settings. <a href="/config">Click here</a> to
				apply one.
			</p>
		{/if}
	{:catch error}
		<p>{error.message}</p>
	{/await}
</div>
