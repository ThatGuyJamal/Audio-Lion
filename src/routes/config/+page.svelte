<script lang="ts">
	import type { AppConfig } from "$lib/types/AppConfig";
	import { onMount } from "svelte";
	import { writable } from "svelte/store";
	import { loadAppConfig } from "$lib/utils/loaders";
	import View from "$lib/components/config/view.svelte";
	import Reset from "$lib/components/config/reset.svelte";
	import FetchSystemFile from "$lib/components/config/FetchSystemFile.svelte";

	// create a writable store for the app config
	const appConfigStore = writable<AppConfig | null>(null);

	// call the loadAppConfig function when the component is mounted
	onMount(() => {
		loadAppConfig(appConfigStore);
	});
</script>

<div>
	<h1>App Configurations</h1>
	<View />
	<Reset />
	<FetchSystemFile />
</div>
