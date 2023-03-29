import Hapi from "@hapi/hapi";
import getPort from "get-port";

const _port = await getPort({ port: [6969, 9696] })
const _server = Hapi.server({
	port: _port,
	host: "localhost",
});

/**
 * Internal HTTP server for our app
 *
 * @class InternalHttpServer
 * @description This class is used to create an internal HTTP server for our app. It is a singleton class and only one instance can be created.
 */
export class InternalHttpServer {
	private static _instance: InternalHttpServer;
	private hapi: Hapi.Server;
  private port: number
  private hostUrl: string

	private constructor() {
		if (InternalHttpServer._instance) {
			throw new Error(
				"Error: Instantiation failed: Use InternalHttpServer.getInstance() instead of new."
			);
		}

		this.hapi = _server;
    this.port = _port

    this.hostUrl = `http://localhost:${this.port}`

		InternalHttpServer._instance = this;
	}

	public static get getInstance(): InternalHttpServer {
		return (
			InternalHttpServer._instance ??
			(InternalHttpServer._instance = new InternalHttpServer())
		);
	}

	public async startServer() {
		this.hapi.route({
			method: "GET",
			path: "/",
			handler: (request, h) => {
				return {
					message: "root",
				};
			},
		});

		await this.hapi.start();
		console.info("Server running on %s", this.hapi.info.uri);
	}

	public getServer() {
		return this.hapi;
	}

	public async stopServer() {
		await this.hapi.stop();
		console.info(`Internal HTTP server stopped...`);
	}

	public async restart() {
		console.warn(`Restarting internal HTTP server...`);
		await this.stopServer();
		await this.startServer();
	}

  /**
   * Gets the root data from the internal HTTP server
   * @returns 
   */
	public async getRoot() {
		return await fetch(
			`${this.hostUrl}/`
		)
	}
}

function AsSingleton<T extends new (...args: any[]) => any>(
	Aclass: T,
	...args: ConstructorParameters<T>
) {
	return class extends Aclass {
		static #instance: InstanceType<T>;
		public static getInstance(): InstanceType<T> {
			if (!this.#instance) {
				this.#instance = new Aclass(...args);
			}
			return this.#instance;
		}
	} as Omit<T, "new"> & { getInstance: () => InstanceType<T> };
}
