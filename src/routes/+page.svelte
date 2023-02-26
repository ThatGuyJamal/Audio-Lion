<script>
	import Config from "$lib/config";
</script>

<div>
	{#if Config.app.under_development}
		<div class="ud">
			<h2>Under Development</h2>
			<p>
				This app is currently under development, click the button below for updates
				information.
			</p>
		</div>
	{/if}

	<h1>Welcome to {Config.app.name}</h1>

	<div class="row">
		<a href={Config.app.github_repo} target="_blank" rel="noreferrer">
			<img src="/lion/icon.svg" class="logo lion" alt="Tauri Logo" />
		</a>
	</div>

	<h2>What is Audio Lion?</h2>
	<p>{Config.app.description}</p>

	<div class="row">
		<button class="btn">
			<a href={Config.app.github_repo} target="_blank" rel="noreferrer">
				Development Progress
			</a>
		</button>

		<button class="btn">
			<a href={Config.app.developer.github} target="_blank" rel="noreferrer">
				Developer Information
			</a>
		</button>

		<button class="btn">
			<a href="/config" rel="noreferrer"> App Config </a>
		</button>

		<button class="btn">
			<a href="/player" rel="noreferrer"> Play Music </a>
		</button>
	</div>

	<p />
</div>

<style>
	.ud {
		background-color: #ff0000;
		color: #ffffff;
		padding: 10px;
	}
	/* 
	h1 {
		color: #646cff;
	} */

	.btn {
		margin-right: 10px;
		margin-left: 10px;
	}
</style>
