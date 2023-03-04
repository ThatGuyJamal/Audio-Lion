<script lang="ts">
	import config from "$lib/config";
	import type { ValidPlatforms } from "$lib/utils/format";
	import { onMount } from "svelte";
	import DiWindows from "svelte-icons/di/DiWindows.svelte";
	import DiLinux from "svelte-icons/di/DiLinux.svelte";
	import DiApple from "svelte-icons/di/DiApple.svelte";
	import { getAppInfo } from "$lib/bindings";

	let os = "unknown" as ValidPlatforms;
	let v = "0.0.0";

	async function getSystemInfo() {
		let info = await getAppInfo();
		os = info.os as ValidPlatforms;
		v = info.version;
	}

	onMount(async () => {
		await getSystemInfo();
	});
</script>

<!-- @see https://daisyui.com/components/modal/#modal-using-label--hidden-checkbox-with-a-close-button-at-corner -->
<input type="checkbox" id="my-modal-3" class="modal-toggle" />
<div class="modal">
	<div class="modal-box relative">
		<div class="mockup-code">
			<label for="my-modal-3" class="btn btn-sm btn-circle absolute right-2 top-2"
				>✕</label
			>
			<h3 class="text-lg font-bold mb-3">Development Information</h3>
			<pre data-prefix=">" class="text-success"><code>Leave a star on github!</code
				></pre>
			<button class="main-page-buttons mb-2">
				<a href={config.app.github_repo} target="_blank" rel="noreferrer">
					GitHub
				</a>
			</button>

			<button class="main-page-buttons">
				<a href={config.app.developer.github} target="_blank" rel="noreferrer">
					The developer
				</a>
			</button>
			<div class="flex justify-center">
				<p class="mt-2 subpixel-antialiased">
					Build Version: {config.app.version_state}-{v}-{os}
				</p>
				{#if os === "windows"}
					<svg class="h-8 w-8 mt-1"><DiWindows /></svg>
				{/if}
				{#if os === "linux"}
					<svg class="h-8 w-8 mt-1"><DiLinux /></svg>
				{/if}
				{#if os === "mac"}
					<svg class="h-8 w-8 mt-1"><DiApple /></svg>
				{/if}
			</div>
		</div>
	</div>
</div>
