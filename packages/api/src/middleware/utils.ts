import type { NextFunction, Request, Response } from "express";
import { CreateAPIReport } from "../db/utils";

export const loggerMiddleware = (
	req: Request,
	res: Response,
	next: NextFunction
) => {
	const start = Date.now();
	res.on("finish", () => {
		const responseTime = Date.now() - start;
		const { method, url } = req;
		const statusCode = res.statusCode;
		console.debug(
			`[DEBUG] > ${req.hostname}${url} - ${method} - ${statusCode} - ${responseTime}ms`
		);
	});
	next();
};

export const errorMiddleware = async (
	err: Error,
	req: Request,
	res: Response,
	_next: NextFunction
) => {
	await CreateAPIReport({
		name: "Error Middleware",
		errorLevel: "Error",
		description: err.message,
		date: new Date(),
		debugInfo: {
			//@ts-ignore
			user_id: req.user?.id ?? undefined,
			caller_function_name: Error.caller.name,
		},
	});

	res.status(400).json({
		message: err.message,
	});
};
