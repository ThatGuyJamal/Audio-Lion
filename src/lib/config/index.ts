import { AudioFileType, type AppConfig } from "$lib/types/AppConfig";

export default {
	app: {
		name: "Audio Lion",
		description: "A simple audio player for your desktop",
		github_repo: "https://github.com/ThatGuyJamal/Audio-Lion",
		developer: {
			name: "ThatGuyJamal",
			github: "https://github.com/ThatGuyJamal",
			solo: "https://solo.to/thatguyjamal",
		},
		community: {
			discord: "https://discord.gg/MSTrBrNaGn",
			landing_page: "",
		},
		under_development: false,
		version_state: "alpha",
		app_config_defaults: {
			audio_directories: [],
			audio_file_types_allowed: [AudioFileType.MP3, AudioFileType.WAV],
		} as AppConfig,
	},
};