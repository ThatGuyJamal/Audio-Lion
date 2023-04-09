<script lang="ts">
	import FaPlay from 'svelte-icons/fa/FaPlay.svelte';
	import FaPause from 'svelte-icons/fa/FaPause.svelte';
	import FaStop from 'svelte-icons/fa/FaStop.svelte';
	import { handleAudio, loadConfig } from '$lib/bindings';

	let appConfig = loadConfig()
		.then((x) => {
			if (x.data) {
				return x.data;
			} else {
				return x.error;
			}
		})
		.catch(() => null);

	$: windowPosition = 0;
	// $: console.log(windowPosition);

	window.addEventListener('scroll', function () {
		windowPosition = window.scrollY;
	});

	const resume = async () => {
		handleAudio("Resume", "")
	};

	const pause = async () => {
		handleAudio("Pause", "")
	};

	const stop = async () => {
		// implement
	};
</script>

<main>
	{#if appConfig !== null}
	<floutBar class="fixed inset-x-0 bottom-0 p-4">
		<div class="rounded-full px- text-white shadow-lg bg-gradient-to-r from-teal-800 to-indigo-500">
			<!-- Play/Pause/Stop buttons -->
			<di class="flex flex-row justify-center">
				<div class="flex flex-row justify-center items-center m-3">
					<button class="btn btn-circle btn-xs glass ml-5 mr-5" on:click={resume}>
						<svg class="h-4 w-4 justify-end">
							<FaPlay />
						</svg>
					</button>
					<button class="btn btn-circle btn-xs glass ml-5 mr-5" on:click={pause}>
						<svg class="h-4 w-4 justify-end">
							<FaPause />
						</svg>
					</button>
					<button class="btn btn-circle btn-xs glass ml-5 mr-5" on:click={stop}>
						<svg class="h-4 w-4 justify-end">
							<FaStop />
						</svg>
					</button>
				</div>
			</di>
		</div></floutBar
	>
{/if}
</main>
