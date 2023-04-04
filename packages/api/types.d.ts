declare module "AudioLionTypes" {
	/**
	 * The application configuration located in the ./src/config.js file.
	 */
	interface AppConfig {
		devMode: boolean;
		port: number;
		mongodbUrl: string;
	}
}
