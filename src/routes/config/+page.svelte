<script lang="ts">
	import { AppConfigLimits, type AppConfig } from "$lib/types/AppConfig";
	import { onMount, tick } from "svelte";
	import { writable } from "svelte/store";
	import { loadAppConfig, resetAppConfig, setAppConfig } from "$lib/utils/tauri";
	import config from "$lib/config";
	import { getCurrentPlatform, isValidDirectory } from "$lib/utils/format";

	// The updated state of the component
	const viewState = writable<AppConfig | null>(null);

	// load the current app config when the component is mounted
	onMount(async () => {
		const load = await loadAppConfig();

		if (load) {
			viewState.set(load);
		} else {
			viewState.set(null);
		}
		await tick();
	});

	// reset the app config to the default values
	const runReset = async () => {
		let result = await resetAppConfig();

		if (result) {
			viewState.set(result);
		} else {
			viewState.set(null);
		}
		await tick();
	};

	$: dirPath = "";
	$: dirInputStatus = "loading" as "invalid" | "valid" | "loading";

	const runNewFolder = async (event: Event) => {
		const config = await loadAppConfig() as AppConfig
		const input = event.target as HTMLInputElement;

		// Makes sure we don't add the same directory twice
		if (config.audio_directories.includes(input.value)) {
			dirPath = "";
			return;
		}

		dirPath = input.value;

		const currentData = await loadAppConfig();

		if (!currentData) {
			throw new Error("No config data found");
		}

		// let checkDir = isValidDirectory(dirPath, getCurrentPlatform());
		// todo - fix the isValidDirectory function
		let checkDir = true;

		if (!checkDir) {
			dirInputStatus = "invalid";
		} else {
			dirInputStatus = "valid";
			currentData.audio_directories.push(dirPath);
			let newData = currentData;
			// console.table(currentData);
			await setAppConfig(currentData);
			viewState.set(newData);
			dirPath = "";
			await tick();
		}
	};
</script>

<div>
	<h1>App Configurations</h1>
	{#await $viewState}
		<p>fetching config file from system...</p>
	{:then result}
		{#if result === null}
			<div>
				<p class="warning">
					It looks like the app config file is missing or corrupted. Please reset it!
				</p>
			</div>
		{:else}
			<div>
				<div>
					{#if result?.audio_directories != null && result?.audio_directories.length > 0}
						<p>Audio Streaming Directories:</p>
						{#each result?.audio_directories as dir}
							<p>{dir}</p>
						{/each}
					{:else}
						<p>No audio streaming directories configured yet.</p>
					{/if}

					{#if result?.audio_file_types_allowed != null && result.audio_file_types_allowed.length > 0}
						<p>Audio File Types Allowed:</p>
						<div>
							{#each result.audio_file_types_allowed as type}
								<li>{type}</li>
							{/each}
						</div>
					{:else}
						<p>
							No audio file filters set, defaulting to {config.app.app_config_defaults.audio_file_types_allowed.join(
								", "
							)}
						</p>
					{/if}
				</div>
			</div>
		{/if}
		{#if result?.audio_directories != null && result?.audio_directories.length <= AppConfigLimits.MAX_AUDIO_DIRECTORIES}
			<h1>Select a folder</h1>
			<p>This path will be used to play your music</p>
			<label for="dir-input">Enter folder directory:</label>
			<input
				type="text"
				id="dir-input"
				on:input={runNewFolder}
				bind:value={dirPath}
			/>
			{#if dirPath}
				{#if dirInputStatus === "invalid"}
					<p class="warning">
						Invalid directory path. Please make sure the folder path is correct.
					</p>
					<p>Example: C:\Users\Bob\Music</p>
				{:else if dirInputStatus === "valid"}
					<p>Selected directory: {dirPath}</p>
				{:else if dirInputStatus === "loading"}
					<p>Checking directory...</p>
				{/if}
			{:else}
				<h2>No path selected</h2>
				<p>Example: C:\Users\Bob\Music</p>
			{/if}
		{:else}
			<p>
				You have reached the maximum number of audio directories allowed. Please
				remove one before adding another.
			</p>
		{/if}
	{:catch error}
		<p>Something went wrong: {error.message}</p>
	{/await}
	<h1>Reset Config Defaults</h1>
	<p>Click the button below to reset the app config to the default values.</p>
	<button on:click={runReset}> Reset Config </button>
</div>

<style>
	.warning {
		background-color: #ff0000;
		color: #ffffff;
		padding: 10px;
	}
</style>
