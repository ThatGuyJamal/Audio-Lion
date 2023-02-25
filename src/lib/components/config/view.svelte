<script lang="ts">
	import type { AppConfig } from "$lib/types/AppConfig";
	import { onMount } from "svelte";
	import { writable } from "svelte/store";
	import config from "$lib/config";
	import { loadAppConfig } from "$lib/utils/loaders";

	// create a writable store for the app config
	const appConfigStore = writable<AppConfig | null>(null);
        
	// call the loadAppConfig function when the component is mounted
	onMount(() => {
		loadAppConfig(appConfigStore);
	});
</script>

<div>
	{#await $appConfigStore}
		<p>fetching config file from system...</p>
	{:then result}
		{#if result === null}
			<div>
				<h1>
					It looks like the app config file is missing or corrupted. Please reset the
					configuration file below!
				</h1>
			</div>
		{:else}
			<div>
				<div>
					{#if result?.audio_directories != null && result?.audio_directories.length > 0}
						<p>Audio Streaming Directories:</p>
						<ul>
							{#each result?.audio_directories as dir}
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
						<p>
							No audio file filters set, defaulting to {config.app.app_config_defaults.audio_file_types_allowed.join(
								", "
							)}
						</p>
					{/if}
				</div>
			</div>
		{/if}
	{:catch error}
		<p>Something went wrong: {error.message}</p>
	{/await}
</div>
