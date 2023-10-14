export interface Config {
	devMode: boolean;
	port: number;
	mongodbUrl: string;
	hostUrl: string;
	sessionSecret: string;
	discord: {
		clientId: string;
		clientSecret: string;
		callbackUrl: string;
		scopes: string[];
		authSuccessUrl: string;
	};
}

const HOST: string = process.env.HOST_URL ?? "http://localhost";
const POST: number = Number(process.env.EXPRESS_PORT) ?? 3000;

export const HostUrl = `${HOST}:${POST}`;

export default {
	devMode: process.env.NODE_ENV ? process.env.NODE_ENV !== "production" : true,
	hostUrl: HOST,
	port: POST,
	mongodbUrl: process.env.MONGODB_URI ?? "mongodb://127.0.0.1:27017/app",
	sessionSecret:
		process.env.EXPRESS_SESSION_SECRET ?? "ADASCZXVZOPXSADASDMXZLQPZCXKMZCA",
	discord: {
		clientId: process.env.DISCORD_CLIENT_ID,
		clientSecret: process.env.DISCORD_CLIENT_SECRET,
		callbackUrl: `${HostUrl}/api/auth/discord/callback`,
		scopes: ["identify", "email", "connections", "guilds", "guilds.join"],
		// the port the app connects to in the background. This can be hardcoded because it doesn't change.
		authSuccessUrl: `http://localhost:1420`, 
	},
} as Config;
