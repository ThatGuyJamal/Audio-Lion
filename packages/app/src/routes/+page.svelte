<script lang="ts">
	import Config from "$lib/config";
	import DevInfo from "../components/popups/dev-info.svelte";
</script>

<DevInfo />

<main>
	<h1 class="mb-3 text-xl font-medium tracking-wide">
		Welcome to {Config.app.name}
	</h1>

	<div class="row">
		<a href={Config.app.github_repo} target="_blank" rel="noreferrer">
			<img src="/lion/icon.svg" class="app-logo" alt="Tauri Logo" />
		</a>
	</div>

	<p class="mt-5 mb-8 text-teal-600">{Config.app.description}</p>

	<div class="alert shadow-lg">
		<div>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				class="stroke-info flex-shrink-0 w-6 h-6"
				><path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
				/></svg
			>
			<div>
				<h3 class="font-bold">Click the small lion to see the menu!</h3>
			</div>
		</div>
	</div>
</main>