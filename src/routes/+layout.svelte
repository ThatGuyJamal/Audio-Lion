<script lang="ts">
	import "../app.css";
	import config from "$lib/config";
</script>

<div class="navbar bg-base-100">
	<div class="flex-1">
		<a class="btn btn-ghost normal-case text-xl" href="/">{config.app.name}</a>
	</div>
	<div class="dropdown dropdown-end">
		<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label tabindex="0" class="btn btn-ghost btn-circle avatar">
			<!-- svelte-ignore a11y-missing-attribute -->
			<div class="w-10 rounded-full">
				<!-- svelte-ignore a11y-missing-attribute -->
				<img src="lion/512px.png" />
			</div>
		</label>
		<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
		<ul
			tabindex="0"
			class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52"
		>
			<li>
				<a class="justify-between" href="/">
					Profile
					<span class="badge">🚧</span>
				</a>
			</li>
			<li><a href="/config">Settings ⚙️</a></li>
			<li><a href="/player">Play Music 🎶</a></li>
			<li><label for="my-modal-3">Dev Information 🧑🏽‍💻</label></li>
			<li>
				<a class="justify-between" href="/">
					Logout
					<span class="badge">🚧</span>
				</a>
			</li>
		</ul>
	</div>
</div>

<!-- The container puts all the rest of the app code into the center of the screen -->
<main class="container">
	<slot />
</main>
