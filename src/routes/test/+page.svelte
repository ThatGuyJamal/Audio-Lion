<script lang="ts">
	import { userStore } from "$lib/stores/UserStore";

	// function set() {
	// 	userStore.setUser("1");
	// }

	let todo: any = [];

	async function testAPI() {
		const options = {
			method: "GET",
		} as RequestInit;

		try {
			const res = await fetch("https://localhost:80/public/health", options);
			todo = await res.json();

			console.log(todo);
		} catch (error) {
			console.log(error);
		}
	}

	testAPI();
</script>

<h1>Test Page</h1>

<br />
<!-- <button on:click={set}>Set User</button> -->

{#each todo as todo (todo.id)}
	<p>{todo.title}</p>
{/each}
