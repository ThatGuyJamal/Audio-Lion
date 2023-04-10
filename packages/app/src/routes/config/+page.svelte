<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { resetConfig, saveConfig, loadConfig } from '$lib/bindings';
	import DevInfo from '../../components/popups/dev-info.svelte';
	import { ApplicationConfigurationState } from '$lib/store';

	// load the current app config when the component is mounted
	onMount(async () => {
		const load = await loadConfig();

		console.debug('loadConfig', load);

		if (load) {
			ApplicationConfigurationState.set(load.data);
		} else {
			ApplicationConfigurationState.set(null);
		}
		await tick();
	});

	$: downloadFolderDirPath = '';
	$: dirInputStatus = 'loading' as 'invalid' | 'valid' | 'loading';

	const runNewFolder = async (event: Event) => {
		const input = event.target as HTMLInputElement;

		downloadFolderDirPath = input.value;

		const currentData = await loadConfig().then((result) => result.data);

		// let checkDir = isValidDirectory(dirPath, getCurrentPlatform());
		// todo - fix the isValidDirectory function
		let checkDir = true;

		if (!checkDir) {
			dirInputStatus = 'invalid';
		} else {
			dirInputStatus = 'valid';

			currentData.download_folder = downloadFolderDirPath;
			let newData = currentData;

			// console.debug("currentData", currentData);
			await saveConfig(newData).catch((err) => {
				console.error(err);
			});

			ApplicationConfigurationState.set(newData);
			downloadFolderDirPath = '';
			await tick();
		}
	};

	const runReset = async () => {
		await resetConfig();
		const load = await loadConfig();

		console.debug('loadConfig', load);

		if (load) {
			ApplicationConfigurationState.set(load.data);
		} else {
			ApplicationConfigurationState.set(null);
		}
		await tick();
	};

	async function handleDownloadClick() {
		const config = await loadConfig();

		if (config) {
			const newConfig = await saveConfig({
				local_audio_folders: config.data.local_audio_folders,
				file_filter_types: config.data.file_filter_types,
				download_folder: null,
				user: config.data.user
			});
			ApplicationConfigurationState.set(newConfig.data);
			await tick();
		} else {
			alert('No config data found');
		}
	}
</script>

<DevInfo />

<main>
	{#await $ApplicationConfigurationState}
		<p>fetching config file from system...</p>
	{:then result}
		{#if result}
			<h1 class="text-xl mb-1">Application Configuration Settings</h1>
			<p>
				You can change the default settings for the application here. These settings will be saved
				to the system and will be loaded when the application starts.
			</p>

			<div class="divider" />
			<h1 class="text-xl mb-2">Download Folder</h1>
			<p>
				You can change the default download folder for the application here. This folder will be
				used to save all downloaded files.
			</p>

			{#if result.download_folder}
				<p>Current path:</p>
				<div class="form-control">
					<label class="label cursor-pointer">
						<span class="label-text mb-1 underline decoration-solid">{result.download_folder}</span>
						<!-- We can hard code the checked (true) value because if its loaded then it must be enabled by default -->
						<input
							type="checkbox"
							checked={true}
							class="checkbox"
							on:click={async () => await handleDownloadClick()}
						/>
					</label>
				</div>
			{:else}
				<label for="dir-input">Paste a folder path:</label>
				<input
					type="text"
					id="dir-input"
					class="file-input file-input-xs max-w-xs mt-4"
					on:input={runNewFolder}
					bind:value={downloadFolderDirPath}
				/>
				<p class="mt-2 text-sm">Example: C:\Users\Bob\Music\Downloads</p>
			{/if}
		{:else}
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
		{/if}

		<div class="divider" />
		<h1 class="text-xl mb-2">Reset Config Defaults</h1>
		<p>Click the button below to reset the app config to the default values.</p>
		<button class="btn mt-3 mb-2" on:click={runReset}> Reset Config </button>
	{:catch error}
		<p>Something went wrong: {error.message}</p>
	{/await}
</main>
