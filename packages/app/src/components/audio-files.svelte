<script lang="ts">
	import { ApplicationConfigurationState, type AudioState } from '$lib/store';

	// the array of audio files to display to the user
	export let audio_files: AudioState[];

	// whether or not to display the audio player
	export let canDisplay: boolean;

	function playAudio(audio: AudioState, index: number) {}
</script>

<main>
	{#await $ApplicationConfigurationState}
		<p>fetching config file from system...</p>
	{:then}
		{#if canDisplay}
			{#each audio_files as file, i}
				<button class="btn btn-sm mb-5 text-teal-600" on:click={() => playAudio(file, i)}>
					{file.name}
				</button>
				<br />
			{/each}
			<div class="stats shadow mb-5 mt-3">
				<div class="stat">
					<div class="stat-title">Total Songs Loaded</div>
					<div class="stat-value">{audio_files.length}</div>
					<!-- <div class="stat-desc"></div> -->
				</div>
			</div>
		{:else}
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
						<p class="font-bold text-red-600 ml-2 mr-2">Invalid Settings</p>
						<div class="text-xs">Missing Music Folders</div>
					</div>
				</div>
				<div class="flex-none">
					<button class="btn btn-sm"><a href="/config">Fix Settings</a></button>
				</div>
			</div>
		{/if}
	{:catch error}
		<p>{error.message}</p>
	{/await}
</main>
