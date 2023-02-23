import type { HandleServerError } from "@sveltejs/kit";

export const handleError = (({ error, event }: any) => {
	return {
		message: "Whoops!",
		code: error?.code ?? "UNKNOWN",
	};
}) satisfies HandleServerError;
