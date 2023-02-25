import { invoke } from "@tauri-apps/api";
import { TauriCommands } from "$lib/types/commands";
import type { AppConfig } from "$lib/types/AppConfig";

/**
 * Loads the current app config. If non is found, it will create a new one
 * @param store The store to update with the new app config
 */
export async function loadAppConfig(): Promise<AppConfig | null> {
	// Get the data from the backend
	// This should not be null, but in case of errors making the file, it will be null
	const result = (await invoke(
		TauriCommands.VIEW_APP_CONFIG
	)) as AppConfig | null;
	// update the store with the new app config data
	return result;
}

export async function getAppConfig(): Promise<AppConfig | null> {
	// Get the data from the backend
	// This should not be null, but in case of errors making the file, it will be null
	const result = (await invoke(
		TauriCommands.VIEW_APP_CONFIG
	)) as AppConfig | null;
	return result;
}

/**
 * Deletes the app config file and reloads the new app config
 * @param store The store to update with the new app config
 * @returns
 */
export async function resetAppConfig(
): Promise<AppConfig> {
	const result = (await invoke(TauriCommands.RESET_APP_CONFIG)) as boolean;
	// After the reset, reload the the app config
	const data = await loadAppConfig() as AppConfig;
	return data;
}

export async function setAppConfig(
	newAppConfig: AppConfig
): Promise<AppConfig> {
	// Save the new app config to the backend
	const result = (await invoke(TauriCommands.SET_APP_CONFIG, {
		audioDirectories: newAppConfig.audio_directories ?? [],
		audioFileTypesAllowed: newAppConfig.audio_file_types_allowed ?? [],
	})) as boolean;
	const data = (await loadAppConfig()) as AppConfig;
	return data;
}
