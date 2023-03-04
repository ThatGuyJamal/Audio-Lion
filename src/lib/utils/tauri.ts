import { invoke } from "@tauri-apps/api";
import { TauriCommands} from "$lib/types/commands";

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
