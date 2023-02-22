<script lang="ts">
	import { TauriCommands } from "$lib/types/commands";
	import { invoke } from "@tauri-apps/api";
	import type { AppConfig } from "$lib/types/config";
	import config from "$lib/config";

	let appConfig: AppConfig = config.app.app_config_defaults;

	// Loads the app config from the Tauri API and returns it as a promise
	export async function loadAppConfig() {
		return await invoke(TauriCommands.VIEW_APP_CONFIG)
			.then((result: unknown) => {
				console.log(result);
				return result as AppConfig;
			})
			.catch((error: unknown) => {
				console.log(error);
				return appConfig;
			});
	}

	// Resets the app config to the default values
	export async function resetAppConfig() {
		return await invoke(TauriCommands.RESET_APP_CONFIG)
			.then((result: unknown) => {
				console.log(result);
				return true;
			})
			.catch((error: unknown) => {
				console.log(error);
				return false;
			});
	}

</script>

<div>
	<!-- Display the loadAppConfig data -->
	<p>
		{#await loadAppConfig()}
			<!-- promise is pending -->
			<p>fetching config file from system...</p>
		{:then result}
			<!-- promise was fulfilled -->
			<p>Caching Enabled: {result.cache_enabled}</p>
		{:catch error}
			<!-- promise was rejected -->
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
</div>
