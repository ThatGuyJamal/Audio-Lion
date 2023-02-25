import { invoke } from "@tauri-apps/api";
import { TauriCommands } from "$lib/types/commands";
import { tick } from "svelte";
import type { AppConfig } from "$lib/types/AppConfig";
import type { Writable } from "svelte/store";

/**
 * Loads the current app config. If non is found, it will create a new one
 * @param store The store to update with the new app config
 */
export async function loadAppConfig(store: Writable<AppConfig | null>): Promise<void> {
	// Get the data from the backend
	// This should not be null, but in case of errors making the file, it will be null
	const result = (await invoke(
		TauriCommands.VIEW_APP_CONFIG
	)) as AppConfig | null;

    store.set(result);
	// update the store with the new app config data
	await tick();
}

/**
 * Deletes the app config file and reloads the new app config
 * @param store The store to update with the new app config
 * @returns 
 */
export async function resetAppConfig(
	store: Writable<AppConfig | null>
): Promise<boolean> {
	const result = await invoke(TauriCommands.RESET_APP_CONFIG) as boolean;
	await tick();
	// After the reset, reload the the app config
	await loadAppConfig(store);
	return result;
}