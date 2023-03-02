<script lang="ts">
	import AudioController from "$lib/components/audio-controller.svelte";
	import Navbar from "$lib/components/navbar.svelte";
	import "../app.css";
</script>

<!-- Application Navigation Menu -->
<Navbar />

<!-- The container puts all the rest of the app code into the center of the screen -->
<main class="container">
	<slot />
</main>

<!-- Controls the audio player views -->

<AudioController />
