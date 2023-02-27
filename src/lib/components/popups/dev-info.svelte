<script>
	import config from "$lib/config";
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
					{config.app.name} GitHub
				</a>
			</button>

			<button class="main-page-buttons">
				<a href={config.app.developer.github} target="_blank" rel="noreferrer">
					The developer
				</a>
			</button>
		</div>
	</div>
</div>
