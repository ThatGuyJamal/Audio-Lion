<script>
	import { page } from "$app/stores";
</script>

<div class="text-center">
	<h1 class="tracking-widest text-gray-500 uppercase">
		404 | {$page.error?.message}
	</h1>
</div>