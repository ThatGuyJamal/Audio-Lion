import mongoose from "mongoose";
import { statisticSchema } from "./db.js";
// import { AppConfig } from "AudioLionTypes";

/**
 * Creates the config object. Runs a few helper functions before returning the config.
 * @param {AppConfig} config The config object
 * @returns {AppConfig} The created config
 */
export function CreateConfig(config) {
  if (!config) throw new Error("No config file passed to CreateConfig");

  validateConfigFile(config);

  if (config.devMode === null || config.devMode === undefined) {
    config.devMode = false;
  }

  console.table(config);

  return config;
}

function validateConfigFile(config) {
  if (!config.port) {
    throw new Error(missing("port"));
  }

  if (config.port < 0 || config.port > 65535) {
    throw new Error(invalid("port"));
  }

  if (config.devMode === null || config.devMode === undefined) {
    throw new Error(missing("devMode"));
  }

  if (typeof config.devMode !== "boolean") {
    throw new Error(invalid("devMode"));
  }

  if (!config.mongodbUrl) {
    throw new Error(missing("mongodbUrl"));
  }
}

export function CreateMongooseConnection(config) {
  mongoose
    .connect(config.mongodbUrl)
    .then(() => {
      console.log("Connected to MongoDB");
    })
    .catch((err) => {
      console.error(err);
    });
}

/**
 * @param {String} caller
 * @returns
 */
function invalid(caller) {
  return `Invalid ${caller} in config file. Please fix it before running the program.`;
}

/**
 * @param {String} caller
 * @returns
 */
function missing(caller) {
  return `Missing ${caller} in config file. Please fix it before running the program.`;
}

export async function GetGlobalStatistics() {
  const data = await statisticSchema.findOne({ findOneId: "global-data" });

  if (!data) await statisticSchema.create({ findOneId: "global-data" });

  return data;
}
