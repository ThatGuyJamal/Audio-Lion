// All commands that can be sent to the Tauri API
export enum TauriCommands {
	VIEW_APP_CONFIG = "view_app_config",
	RESET_APP_CONFIG = "reset_app_config",
	SET_APP_CONFIG = "set_app_config",
	GET_AUDIO_FILES = "get_audio_files",
	PLAY_AUDIO_FILE = "play_audio_file",
	PAUSE_AUDIO_FILE = "pause_audio_file",
	RESUME_AUDIO_FILE = "resume_audio_file",
	STOP_AUDIO_FILE = "stop_audio_file",
	SKIP_AUDIO_FILE = "skip_audio_file",
	GET_APP_INFO = "get_app_info",
}

/** The app information returned from the backend */
export type AppInfo = {
	os: string;
	name: string,
	version: string,
	description: string,
}