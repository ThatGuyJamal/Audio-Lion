<script>
  import { page } from '$app/stores';
</script>

<h1>{$page.error?.message}</h1>

<button on:click={
    () => {
        window.location.href = '/';
    }
}>
    Go back
</button>