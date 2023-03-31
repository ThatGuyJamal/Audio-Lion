import http from "node:http";

export interface UserSchema {
	// _id: ObjectId - only needed on the server side but the type does exist
	user_id: string;
	username: string;
	hashPassword: string;
	email: string;
	createdAt: number;
	/** The user is considered "deleted"*/
	tombStoned: boolean;
	tombedAt: number | null;
	premium: UserPremiumSchema | null;
	app: AppSchema | null;
}

export interface UserPremiumSchema {
	active: boolean;
	expires: Date;
}

export interface AppSchema {
	errors_reported: number;
}

class IUserStore {
	/** User is null until sign */
	user: UserSchema | null;

	public constructor() {
		this.user = null;
	}

	public setUser(userId: string) {
		const options = {
			hostname: "http://localhost:80",
			path: `/internals/user/${userId}`,
			method: "GET",
		};

		const req = http.request(options, (res) => {
			console.log(`STATUS: ${res.statusCode}`);
			console.log(`HEADERS: ${JSON.stringify(res.headers)}`);

			res.on("data", (d: unknown) => {
				console.table(`BODY: ${d}`);
			});

			res.on("end", () => {
				console.log("No more data in response.");
			});
		});

		req.on("error", (error) => {
			console.error(error);
		});
	}

	public getUser() {
		return this.user;
	}
}

const userStore = new IUserStore();

export { userStore };
