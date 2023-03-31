import type { PageLoad } from "./$types";

interface HealthCheck {
    ok: boolean;
    message: string;
    data: any
}

export const load = (async ({ fetch, params }: any) => {
	const res = await fetch("https://localhost:8080/public/health", {
        method: "GET",
    });

	console.log(res);

	return res.json() as unknown as HealthCheck;
}) satisfies PageLoad;
