<script lang="ts">
	import { onMount } from "svelte";
	import { writable } from "svelte/store";
	import { resetAppConfig, loadAppConfig } from "$lib/utils/loaders";
	import type { AppConfig } from "$lib/types/AppConfig";

	// create a writable store for the app config
	const appConfigStore = writable<AppConfig | null>(null);

	// call the loadAppConfig function when the component is mounted
	onMount(() => {
		loadAppConfig(appConfigStore);
	});

	function runReset() {
		resetAppConfig(appConfigStore).then((result) => {
			if (result) {
				console.log("Config reset to default values");
			} else {
				console.log("Config reset failed");
			}
		});
	}
</script>

<div>
	<h1>Reset Config Defaults</h1>
	<p>Click the button below to reset the app config to the default values.</p>
	<button on:click={() => runReset()}> Reset Config </button>
</div>
