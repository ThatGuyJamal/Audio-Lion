<script lang="ts">
	import FaArrowUp from "svelte-icons/fa/FaArrowUp.svelte";

	import FaPlay from "svelte-icons/fa/FaPlay.svelte";
	import FaPause from "svelte-icons/fa/FaPause.svelte";
	import FaStop from "svelte-icons/fa/FaStop.svelte";
	import { loadConfig } from "$lib/bindings";

	let appConfig = loadConfig()
		.then((x) => {
			if (x.data) {
				return x.data;
			} else {
				return x.error
			}
		})
		.catch(() => null);

	$: windowPosition = 0;
	// $: console.log(windowPosition);

	window.addEventListener("scroll", function () {
		windowPosition = window.scrollY;
	});

	// function scrollToTop() {
	// 	window.scrollTo({ top: 0, behavior: "smooth" });
	// }

	const resume = async () => {
		// implement
	};

	const pause = async () => {
		// implement
	};

	const stop = async () => {
		// implement
	};
</script>

{#if appConfig !== null}
	<floutBar class="fixed inset-x-0 bottom-0 p-4">
		<div
			class="rounded-full px- text-white shadow-lg bg-gradient-to-r from-teal-800 to-indigo-500"
		>
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

			<!-- Todo get this working. -->
			<!-- <div class="flex justify-center">
				{#if windowPosition > 100}
					<button
						class="btn btn-circle glass btn-xs -mr-80"
						on:click|preventDefault={scrollToTop}
					>
						<svg class=" h-5 w-5">
							<FaArrowUp />
						</svg>
					</button>
				{/if}
			</div> -->
		</div></floutBar
	>
{/if}
