<script lang="ts">
	import { AppConfigLimits, type AppConfig } from "$lib/types/AppConfig";
	import { onMount, tick } from "svelte";
	import { writable } from "svelte/store";
	import { loadAppConfig, resetAppConfig, setAppConfig } from "$lib/utils/tauri";
	import config from "$lib/config";
 import DevInfo from "$lib/components/popups/dev-info.svelte";
	// import { getCurrentPlatform, isValidDirectory } from "$lib/utils/format";

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
		const config = (await loadAppConfig()) as AppConfig;
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

<DevInfo />

<main>
	<h1 class="text-2xl mb-6">App Configurations</h1>
	{#await $viewState}
		<p>fetching config file from system...</p>
	{:then result}
		{#if result === null}
			<div>
				It looks like the configuration file is missing or corrupted!
				<div class="dropdown dropdown-end">
					<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
					<!-- svelte-ignore a11y-label-has-associated-control -->
					<label tabindex="0" class="btn btn-circle btn-ghost btn-xs text-info">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							class="w-4 h-4 stroke-current"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
					</label>
					<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
					<!-- svelte-ignore a11y-label-has-associated-control -->
					<div
						tabindex="0"
						class="card compact dropdown-content shadow bg-base-100 rounded-box w-64"
					>
						<div class="card-body">
							<h2 class="card-title text-center">Need Support?</h2>
							<p class="text-red-400">Please contact the app developers on discord.</p>
						</div>
					</div>
				</div>
			</div>
		{:else}
			<div>
				<div>
					{#if result?.audio_directories != null && result?.audio_directories.length > 0}
						<div class="divider" />
						<h1 class="text-xl mb-1">Audio Streaming Directories:</h1>
						{#each result?.audio_directories as dir}
							<p class="mb-1 underline decoration-solid">{dir}</p>
						{/each}
					{:else}
						<p class="text-amber-200">
							No audio streaming directories configured yet.
						</p>
					{/if}
					<div class="divider" />
					{#if result?.audio_file_types_allowed != null && result.audio_file_types_allowed.length > 0}
						<h1 class="text-xl mb-1">Audio File Type(s) Filter</h1>
						<p>
							The current audio file types below are supported by {config.app.name}:
						</p>
						{#each result.audio_file_types_allowed as fileExtensionType}
							<p>{fileExtensionType}</p>
						{/each}
					{:else}
						<p>
							No audio file filters set, defaulting to {config.app.app_config_defaults
								.audio_file_types_allowed}
						</p>
					{/if}
				</div>
			</div>
			<div class="divider" />
			{#if result?.audio_directories != null && result?.audio_directories.length <= AppConfigLimits.MAX_AUDIO_DIRECTORIES}
				<h1 class="text-xl mb-1">Select Music Folders</h1>
				<p class="mb-2">This path will be used to play your music</p>
				<label for="dir-input">Enter folder directory:</label>
				<input
					type="text"
					id="dir-input"
					class="file-input file-input-xs max-w-xs"
					on:input={runNewFolder}
					bind:value={dirPath}
				/>
				{#if dirPath}
					{#if dirInputStatus === "invalid"}
						<p class="warning">
							Invalid directory path. Please make sure the folder path is correct!
						</p>
						<p>Example: C:\Users\Bob\Music</p>
					{:else if dirInputStatus === "valid"}
						<p>Selected directory: {dirPath}</p>
					{:else if dirInputStatus === "loading"}
						<p>Checking directory...</p>
					{/if}
				{:else}
					<p class="mt-2">Example: C:\Users\Bob\Music</p>
				{/if}
			{:else}
				<input
					type="file"
					placeholder="Max Folders reached"
					class="file-input  file-input-xs max-w-xs"
					disabled
				/>
			{/if}
			<div class="divider" />
			<h1 class="text-xl mb-2">Reset Config Defaults</h1>
			<p>Click the button below to reset the app config to the default values.</p>
			<button class="btn mt-3" on:click={runReset}> Reset Config </button>
		{/if}
	{:catch error}
		<p>Something went wrong: {error.message}</p>
	{/await}
</main>
