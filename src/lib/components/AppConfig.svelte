<script lang="ts">
	import { TauriCommands } from "$lib/types/commands";
	import { invoke } from "@tauri-apps/api";
	import type { AppConfig } from "$lib/types/AppConfig";
	import { tick, onMount } from "svelte";
	import { writable } from "svelte/store";
 import FetchSystemFile from "./utils/FetchSystemFile.svelte";
 import config from "$lib/config";

	// create a writable store for the app config
	const appConfigStore = writable<AppConfig | null>(null);

	// Loads the app config from the Tauri API and updates the store
	async function loadAppConfig(): Promise<void> {
		// Get the data from the backend
		// This should not be null, but in case of errors making the file, it will be null
		const result = (await invoke(
			TauriCommands.VIEW_APP_CONFIG
		)) as AppConfig | null;
		// update the store with the new app config data
		appConfigStore.set(result);
		await tick();
	}

	// call the loadAppConfig function when the component is mounted
	onMount(() => {
		loadAppConfig();
	});

	// Resets the app config to the default values
	export async function resetAppConfig(): Promise<boolean> {
		const result = (await invoke(TauriCommands.RESET_APP_CONFIG)) as boolean;
		await tick();
		// reload the app config data after resetting
		await loadAppConfig();
		return result;
	}
</script>

<div>
	<!-- Display the app config data from the store -->
	<p>
		{#await $appConfigStore}
			<p>fetching config file from system...</p>
		{:then result}
			<div>
				<div>
					{#if result?.audio_directories != null && result?.audio_directories.length > 0}
						<p>Audio Streaming Directories:</p>
						<ul>
							{#each result?.audio_directories as dir} 999 x 790
								<li>{dir}</li>
							{/each}
						</ul>
					{:else}
						<p>No audio streaming directories configured yet.</p>
					{/if}

					{#if result?.audio_file_types_allowed != null && result.audio_file_types_allowed.length > 0}
						<p>Audio File Types Allowed:</p>
						<ul>
							{#each result.audio_file_types_allowed as type}
								<li>{type}</li>
							{/each}
						</ul>
					{:else}
						<p>No audio file filters set, defaulting to {config.app.app_config_defaults.audio_file_types_allowed.join(", ")}</p>
					{/if}
				</div>
			</div>
		{:catch error}
			<p>Something went wrong: {error.message}</p>
		{/await}
	</p>
	<br />
	<h1>Reset Config Defaults</h1>
	<p>Click the button below to reset the app config to the default values.</p>
	<button
		on:click={async () =>
			await resetAppConfig().then((result) => {
				if (result) {
					console.log("Config reset to default values");
				} else {
					console.log("Config reset failed");
				}
			})}
	>
		Reset Config
	</button>
	<br>
	<br>
	<br>
	<h1>Select a file path</h1>
	<p>This path will be used to play your music</p>
    <FetchSystemFile />
</div>
