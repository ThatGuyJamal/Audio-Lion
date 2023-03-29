<script lang="ts">
	import { AppConfigLimits } from "$lib/types/AppConfig";
	import { onMount, tick } from "svelte";
	import DevInfo from "$lib/components/popups/dev-info.svelte";
	import { ApplicationConfigurationState } from "$lib/stores/ConfigStore";
	import { resetConfig, saveConfig, loadConfig } from "$lib/bindings";

	// load the current app config when the component is mounted
	onMount(async () => {
		const load = await loadConfig();

		console.debug("loadConfig", load);

		if (load) {
			ApplicationConfigurationState.set(load.data);
		} else {
			ApplicationConfigurationState.set(null);
		}
		await tick();
	});

	const runReset = async () => {
		await resetConfig();
		const load = await loadConfig();

		console.debug("loadConfig", load);

		if (load) {
			ApplicationConfigurationState.set(load.data);
		} else {
			ApplicationConfigurationState.set(null);
		}
		await tick();
	};

	$: dirPath = "";
	$: dirInputStatus = "loading" as "invalid" | "valid" | "loading";

	const runNewFolder = async (event: Event) => {
		const config = await loadConfig();
		const input = event.target as HTMLInputElement;

		// Makes sure we don't add the same directory twice
		if (config.data.audio_directories.includes(input.value)) {
			dirPath = "";
			return;
		}

		dirPath = input.value;

		const currentData = await loadConfig().then((result) => result.data);

		// let checkDir = isValidDirectory(dirPath, getCurrentPlatform());
		// todo - fix the isValidDirectory function
		let checkDir = true;

		if (!checkDir) {
			dirInputStatus = "invalid";
		} else {
			dirInputStatus = "valid";

			currentData.audio_directories.push(dirPath);
			let newData = currentData;

			// console.debug("currentData", currentData);
			await saveConfig({
				audio_device_name: newData.audio_device_name,
				audio_directories: newData.audio_directories,
				audio_file_types_allowed: newData.audio_file_types_allowed,
			}).catch((err) => {
				console.error(err);
			});

			ApplicationConfigurationState.set(newData);
			dirPath = "";
			await tick();
		}
	};

	// Handles the checkbox for each folder
	async function handleFolderChecks(dir: string) {
		const config = await loadConfig();

		if (config) {
			let index = config.data.audio_directories.indexOf(dir);
			if (index > -1) {
				config.data.audio_directories.splice(index, 1);
			}
			const newConfig = await saveConfig({
				audio_device_name: config.data.audio_device_name,
				audio_directories: config.data.audio_directories,
				audio_file_types_allowed: config.data.audio_file_types_allowed,
			});
			ApplicationConfigurationState.set(newConfig.data);
			await tick();
		} else {
			alert("No config data found");
		}
	}
</script>

<DevInfo />

<main>
	{#await $ApplicationConfigurationState}
		<p>fetching config file from system...</p>
	{:then result}
		{#if result === null}
			<div>
				It looks like the configuration file is missing or corrupted!
				<div class="dropdown dropdown-end animate-pulse">
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
						<h1 class="text-xl mb-1">Audio Streaming Directories:</h1>
						{#each result?.audio_directories as dir}
							<div class="form-control">
								<label class="label cursor-pointer">
									<span class="label-text mb-1 underline decoration-solid">{dir}</span>
									<!-- We can hard code the checked (true) value because if its loaded then it must be enabled by default -->
									<input
										type="checkbox"
										checked={true}
										class="checkbox"
										on:click={() => handleFolderChecks(dir)}
									/>
								</label>
							</div>
						{/each}
						<p class="text-sm text-red-400">Uncheck to remove a folder</p>
					{:else}
						<p class="text-amber-200">
							No audio streaming directories configured yet.
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
			<button class="btn mt-3 mb-2" on:click={runReset}> Reset Config </button>
		{/if}
	{:catch error}
		<p>Something went wrong: {error.message}</p>
	{/await}
</main>
