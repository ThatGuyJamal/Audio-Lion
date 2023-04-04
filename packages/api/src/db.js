import mongoose from "mongoose";

const stats = mongoose.Schema({
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

export const statisticSchema = mongoose.model("statistics", stats);

const reports = mongoose.Schema({
	name: {
		type: String,
		required: false,
	},
	errorLevel: {
		type: String,
		required: false,
	},
	description: {
		type: String,
		required: false,
	},
	date: {
		type: Date,
		required: false,
	},
	debugInfo: {
		type: Object,
		required: false,
	},
});

export const reportSchema = mongoose.model("reports", reports);
