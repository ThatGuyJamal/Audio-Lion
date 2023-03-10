import { invoke } from "@tauri-apps/api";
import { TauriCommands, type AppInfo } from "$lib/types/commands";
import type { AppConfig, AudioFileType } from "$lib/types/AppConfig";
import config from "$lib/config";

/**
 * Loads the current app config. If non is found, it will create a new one
 * @param store The store to update with the new app config
 */
export async function loadAppConfig(): Promise<AppConfig | null> {
	// Get the data from the backend
	// This should not be null, but in case of errors making the file, it will be null
	return await invoke<AppConfig | null>(TauriCommands.VIEW_APP_CONFIG);
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
	return await invoke<AppConfig>(TauriCommands.RESET_APP_CONFIG);
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
		audioDeviceName: newAppConfig.audio_device_name, 
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

export type PlayAudioParams = {
	filePath: String;
};

export enum AudioCommands {
	PLAY = "Play",
	PAUSE = "Pause",
	RESUME = "Resume",
	SKIP = "Skip",
	STOP = "Stop",
}

export interface InputParams {
	command: AudioCommands;
	player_path?: string;
}

export type AudioCommandResult = {
	success: boolean;
	is_paused: boolean;
};

/**
 * @returns {AudioCommandResult} The result of the command
 */
export async function handle_audio_input(
	params: InputParams
): Promise<AudioCommandResult> {
	return await invoke<AudioCommandResult>(TauriCommands.HANDLE_AUDIO_INPUT, {
		command: params.command,
		playerPath: params.player_path,
	});
}

/**
 * @returns	The app info from the backend
 */
export async function getAppInfo(): Promise<AppInfo> {
	return await invoke<AppInfo>(TauriCommands.GET_APP_INFO).then((data) => data);
}
