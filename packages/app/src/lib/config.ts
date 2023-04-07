import type { AppConfig } from '$lib/bindings';

export default {
	app: {
		name: 'Audio Lion',
		description: 'A simple audio player for your desktop',
		github_repo: 'https://github.com/audio-lion-dev/Audio-Lion',
		developer: {
			name: 'ThatGuyJamal',
			github: 'https://github.com/ThatGuyJamal',
			solo: 'https://solo.to/thatguyjamal'
		},
		community: {
			discord: 'https://discord.gg/MSTrBrNaGn',
			landing_page: ''
		},
		under_development: false,
		version_state: 'alpha',
		app_config_defaults: {
			local_audio_folders: [],
			file_filter_types: [],
			user: null
		} as AppConfig
	}
};


export enum AppConfigLimits {
	MAX_AUDIO_DIRECTORIES = 2
}