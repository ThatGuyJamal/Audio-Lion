<script>
	import Home from "$lib/components/Home.svelte";
	import Config from "../lib/config";
</script>

{#if Config.app.under_development}
	<div class="ud">
		<h2>Under Development</h2>
		<p>
			This app is currently under development, click the button below for updates information.
		</p>
	</div>
{/if}

<h1>Welcome to {Config.app.name}</h1>

<div class="row">
	<a href="{Config.app.github_repo}" target="_blank" rel="noreferrer">
		<img src="/lion/icon.svg" class="logo lion" alt="Tauri Logo" />
	</a>
</div>

<Home />

<style>
	.ud {
		background-color: #ff0000;
		color: #ffffff;
		padding: 10px;
	}
</style>
