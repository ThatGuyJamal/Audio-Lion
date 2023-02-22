<script lang="ts">
	import Config from "../config";
</script>

<div>
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
			<a href="/config" rel="noreferrer">
				App Config
			</a>
		</button>
	</div>

	<p />
</div>

<style>
	.btn {
		margin-right: 10px;
		margin-left: 10px;
	}
</style>
