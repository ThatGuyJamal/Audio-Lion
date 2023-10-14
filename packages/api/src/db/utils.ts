import mongoose from "mongoose";
import type { Config } from "../configs/index.js";
import { Report, reportSchema, statisticSchema } from "./schemas.js";

/**
 * Creates the mongoose connection
 * @param {Config} config
 * @returns {void}
 */
export function CreateMongooseConnection(config: Config): void {
	mongoose
		.connect(config.mongodbUrl)
		.then(() => {
			console.log("Connected to MongoDB");
		})
		.catch((err) => {
			console.error(err);
		});
}


export async function GetGlobalStatistics() {
	const data = await statisticSchema.findOne({ findOneId: "global-data" });

	if (!data) await statisticSchema.create({ findOneId: "global-data" });

	return data;
}

export async function CreateAPIReport(report: Report): Promise<void> {
	await reportSchema.create(report);
}