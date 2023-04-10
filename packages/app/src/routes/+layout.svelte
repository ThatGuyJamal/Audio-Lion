<script lang="ts">
	import '../app.css';
	import AudioController from '../components/audio-controller.svelte';
	import Navbar from '../components/navbar.svelte';
</script>

<!-- Application Navigation Menu -->
<Navbar />

<!-- The container puts all the rest of the app code into the center of the screen -->
<main class="row py-20 text-center">
	<slot />
</main>

<!-- Controls the audio player views. -->

<!-- <AudioController /> -->
