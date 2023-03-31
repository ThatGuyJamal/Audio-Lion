<script lang="ts">
  import type { PageData } from './$types';

  export let result: PageData;
</script>

<h1>Ok: {result.ok}</h1>
<div>Message: {@html result.message}</div>
<div>Data: {result.data}</div>