/**
 *  Code Goblin - A discord bot for programmers.

 Copyright (C) 2022, ThatGuyJamal and contributors
 This program is free software: you can redistribute it and/or modify
 it under the terms of the GNU Affero General Public License as
 published by the Free Software Foundation, either version 3 of the
 License, or (at your option) any later version.
 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 GNU Affero General Public License for more details.
 */

import winston from "winston";
import { format } from "logform";

class ILogger {
  private instance;
  private readonly IsInDevelopmentMode: boolean;

  public constructor() {
    this.IsInDevelopmentMode = import.meta.env.DEV;

    this.instance = winston.createLogger({
      level: "info",
      format: winston.format.json(),
      defaultMeta: { service: "internal-logs" },
      transports: [
        new winston.transports.File({
          filename: "./logs/info.log",
          level: "info",
        }),
        new winston.transports.File({
          filename: "./logs/error.log",
          level: "error",
        }),
        new winston.transports.File({
          filename: "./logs/debug.log",
          level: "debug",
        }),
        new winston.transports.File({
          filename: "./logs/warn.log",
          level: "warn",
        }),
      ],
    });

    if (this.IsInDevelopmentMode) {
      this.instance.add(
        new winston.transports.Console({
          format: winston.format.simple(),
        })
      );
    }

    this.instance.format = format.combine(
      format.colorize(),
      format.timestamp(),
      format.align(),
      format.printf(
        (info) => `${info.timestamp} ${info.level}: ${info.message}`
      )
    );

    this.instance.exceptions.handle(
      new winston.transports.File({ filename: "./logs/exceptions.log" })
    );
  }

  /**
     *{
        emerg: 0,
        alert: 1,
        crit: 2,
        error: 3,
        warning: 4,
        notice: 5,
        info: 6,
        debug: 7
        }
     */

  /**
   * Log a message at the 'info' level
   * @param message
   * @param args
   */
  public info(message: string, ...args: any[]) {
    if (this.IsInDevelopmentMode) this.instance.info(message, ...args);
  }

  /**
   * Log a message at the 'error' level
   * @param message
   * @param args
   * @returns
   */
  public error(message: any, ...args: any[]) {
    if (this.IsInDevelopmentMode) this.instance.error(message, ...args);
  }

  /**
   * Log a message at the 'error' level
   * @param message
   * @param args
   * @returns
   */
  public warn(message: string, ...args: any[]) {
    if (this.IsInDevelopmentMode) this.instance.warn(message, ...args);
  }

  /**
   * Log a message at the 'debug' level
   * @param message
   * @param args
   * @returns
   */
  public debug(message: string, ...args: any[]) {
    if (this.IsInDevelopmentMode) this.instance.debug(message, ...args);
  }

  /**
   * Log a message at the 'crit' level
   * @param message
   * @param args
   * @returns
   */
  public crit(message: string, ...args: any[]) {
    if (this.IsInDevelopmentMode) this.instance.crit(message, ...args);
  }

  /**
   * Log a message at the 'notice' level
   * @param message
   * @param args
   * @returns
   */
  public notice(message: string, ...args: any[]) {
    if (this.IsInDevelopmentMode) this.instance.notice(message, ...args);
  }
}

const Logger = new ILogger();

export default Logger;