export type AppConfig = {
	/**
	 * The directories where the app will look for audio files.
	 *
	 * This is set by the user in the app's settings.
	 */
	audio_directories: string[];
	/**
	 * The file types that the app will look for in the audio directories.
	 *
	 * These can be file types of "mp3" or "wav" as of now.
	 */
	audio_file_types_allowed: string[];
};

export enum AudioFileType {
    MP3 = "mp3",
    WAV = "wav",
}
