import { Schema, SchemaTypes, model } from "mongoose";

export interface Stats {
	findOneId: string;
	online_users: number;
	total_users: number;
	downloads: number;
}

const stats = new Schema<Stats>({
	findOneId: {
		type: String,
		unique: true,
		default: "findOne",
	},
	online_users: {
		type: Number,
		required: false,
		default: 0,
	},
	total_users: {
		type: Number,
		required: false,
		default: 0,
	},
	downloads: {
		type: Number,
		required: false,
		default: 0,
	},
});

export const statisticSchema = model("statistics", stats);

export interface Report {
	name: string;
	errorLevel: "Error" | "Warning" | "Info";
	description: string;
	date: Date;
	debugInfo: {
		user_id?: string;
		caller_function_name?: string;
		caller_file_name?: string;
		caller_line_number?: number;
		api_version?: string;
	}
}

const reports = new Schema({
	name: {
		type: SchemaTypes.String,
		required: false,
	},
	errorLevel: {
		type: SchemaTypes.String,
		required: false,
	},
	description: {
		type: SchemaTypes.String,
		required: false,
	},
	date: {
		type: SchemaTypes.Date,
		required: false,
	},
	debugInfo: {
		type: Object,
		required: false,
	},
});

export const reportSchema = model("reports", reports);

/** The user object saved in our api session */
export interface User {
	id: string;
	discordId: string;
	accessToken: string;
	refreshToken: string;
}

const userSchema = new Schema<User>({
	discordId: {
		type: SchemaTypes.String,
		required: true,
		unique: true,
	},
	accessToken: { type: SchemaTypes.String, required: true },
	refreshToken: { type: SchemaTypes.String, required: true },
});

export const UserSchema = model("users", userSchema);
