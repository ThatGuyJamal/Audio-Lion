<script>
  import { page } from '$app/stores';
</script>

<h1 class="text-xl text-red-600">{$page.error?.message}</h1>