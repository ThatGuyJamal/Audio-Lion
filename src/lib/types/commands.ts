// All commands that can be sent to the Tauri API
export enum TauriCommands {
	VIEW_APP_CONFIG = "view_app_config",
	RESET_APP_CONFIG = "reset_app_config",
	SET_APP_CONFIG = "set_app_config",
	GET_AUDIO_FILES = "get_audio_files",
	HANDLE_AUDIO_INPUT = "handle_audio_input",
	GET_APP_INFO = "get_app_info",
}
