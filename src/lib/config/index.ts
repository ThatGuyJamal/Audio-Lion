import type { AppConfig } from "$lib/types/config";

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
		under_development: false,
		version: "0.0.1",
		app_config_defaults: {
			cache_enabled: true,
		} as AppConfig
	},
};