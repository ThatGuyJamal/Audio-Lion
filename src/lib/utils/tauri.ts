import { invoke } from "@tauri-apps/api";
import { TauriCommands } from "$lib/types/commands";
import type { AppConfig, AudioFileType } from "$lib/types/AppConfig";
import { getDirectoryPath } from "./format";

/**
 * Loads the current app config. If non is found, it will create a new one
 * @param store The store to update with the new app config
 */
export async function loadAppConfig(): Promise<AppConfig | null> {
	// Get the data from the backend
	// This should not be null, but in case of errors making the file, it will be null
	const result = await invoke<AppConfig | null>(TauriCommands.VIEW_APP_CONFIG);
	// update the store with the new app config data
	return result;
}

/**
 * @returns The current app config
 */
export async function getAppConfig(): Promise<AppConfig | null> {
	return await invoke<AppConfig | null>(TauriCommands.VIEW_APP_CONFIG);
}

/**
 * Deletes the app config file and reloads the new app config
 * @param store The store to update with the new app config
 * @returns
 */
export async function resetAppConfig(): Promise<AppConfig> {
	const result = await invoke<boolean>(TauriCommands.RESET_APP_CONFIG);
	// After the reset, reload the the app config
	const data = (await loadAppConfig()) as AppConfig;
	return data;
}

/**
 * @param newAppConfig The data to set the app config to
 * @returns The new app config
 */
export async function setAppConfig(
	newAppConfig: AppConfig
): Promise<AppConfig> {
	// Save the new app config to the backend
	await invoke<boolean>(TauriCommands.SET_APP_CONFIG, {
		audioDirectories: newAppConfig.audio_directories ?? [],
		audioFileTypesAllowed: newAppConfig.audio_file_types_allowed ?? [],
	});
	const data = await loadAppConfig();
	return data!;
}

/**
 * Gets all the audio files in the audio directories
 * @param fileType The type of audio file to get
 * @returns {String[]} An array of file paths
 */
export async function getAudioFiles(
	fileType: AudioFileType
): Promise<string[]> {
	const result = await invoke<string[]>(TauriCommands.GET_AUDIO_FILES, {
		audioFileType: fileType,
	}).catch(() => {
		// console.error(err);
		return [];
	});

	if (!result || result.length === 0) return [];

	return result;
}

type PlayAudioFile = {
	file_path: string;
	file_type: AudioFileType;
	/** The index of the file. This is used as a ref when the backend searches for the file later. */
	file_index: number;
};

/**
 * Sends a command to the backend to play an audio file
 * @returns
 */
export async function playAudioFile({
	file_path,
	file_type,
	file_index,
}: PlayAudioFile): Promise<boolean> {
	return await invoke<boolean>(TauriCommands.PLAY_AUDIO_FILE, {
		filePath: file_path,
		fileType: file_type,
		fileIndex: file_index,
	}).catch(() => {
		return false;
	});
}

// TODO - Implement these
export async function pauseAudioFile(): Promise<boolean> {
	return await invoke<boolean>(TauriCommands.PAUSE_AUDIO_FILE).catch(() => {
		return false;
	});
}

// TODO - Implement these
export async function resumeAudioFile(): Promise<boolean> {
	return await invoke<boolean>(TauriCommands.RESUME_AUDIO_FILE).catch(() => {
		return false;
	});
}

// TODO - Implement these
export async function stopAudioFile(): Promise<boolean> {
	return await invoke<boolean>(TauriCommands.STOP_AUDIO_FILE).catch(() => {
		return false;
	});
}
