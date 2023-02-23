<script lang="ts">
    import { TauriCommands } from "$lib/types/commands";
    import { invoke } from "@tauri-apps/api";
    import type { AppConfig } from "$lib/types/config";
    import { tick, onMount } from "svelte";
    import { writable } from 'svelte/store';

    // create a writable store for the app config
    const appConfigStore = writable<AppConfig | null>(null);

    // Loads the app config from the Tauri API and updates the store
    async function loadAppConfig(): Promise<void> {
        // Get the data from the backend
        // This should not be null, but in case of errors making the file, it will be null
        const result = (await invoke(
            TauriCommands.VIEW_APP_CONFIG
        )) as AppConfig | null;

        // update the store with the new app config data
        appConfigStore.set(result);

        await tick();
    }

    // call the loadAppConfig function when the component is mounted
    onMount(() => {
        loadAppConfig();
    });

    // Resets the app config to the default values
    export async function resetAppConfig(): Promise<boolean> {
        const result = (await invoke(TauriCommands.RESET_APP_CONFIG)) as boolean;

        await tick();

        // reload the app config data after resetting
        await loadAppConfig();

        return result;
    }
</script>

<div>
    <!-- Display the app config data from the store -->
    <p>
        {#await $appConfigStore}
            <!-- promise is pending -->
            <p>fetching config file from system...</p>
        {:then result}
            <!-- promise was fulfilled -->
            <p>Caching Enabled: {result?.cache_enabled ?? "No data"}</p>
        {:catch error}
            <!-- promise was rejected -->
            <p>Something went wrong: {error.message}</p>
        {/await}
    </p>
    <br />
    <h1>Reset Config Defaults</h1>
    <p>Click the button below to reset the app config to the default values.</p>
    <button
        on:click={async () =>
            await resetAppConfig().then((result) => {
                if (result) {
                    console.log("Config reset to default values");
                } else {
                    console.log("Config reset failed");
                }
            })}
    >
        Reset Config
    </button>
</div>
