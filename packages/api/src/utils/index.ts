import type { Config } from "../configs/index.js";

/** Creates a new config but validates the props first. */
export function CreateConfig(config: Config): Config {
  if (!config) throw new Error("No config file passed to CreateConfig");

  validateConfigFile(config);

  if (config.devMode === null || config.devMode === undefined) {
    config.devMode = false;
  }

  console.debug(config);

  return config;
}

function validateConfigFile(config: Config) {
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

  if (!config.hostUrl) {
    throw new Error(missing("hostUrl"));
  }

  if (!config.mongodbUrl.startsWith("mongodb://")) {
    throw new Error(invalid("mongodbUrl"));
  }

  if (!config.hostUrl.startsWith("http://")) {
    throw new Error(invalid("hostUrl"));
  }

  if(!config.discord) {
    throw new Error(missing("discord"));
  }
}

/**
 * @param {String} caller
 * @returns
 */
function invalid(caller: string) {
  return `Invalid ${caller} in config file. Please fix it before running the program.`;
}

/**
 * @param {String} caller
 * @returns
 */
function missing(caller: string) {
  return `Missing ${caller} in config file. Please fix it before running the program.`;
}