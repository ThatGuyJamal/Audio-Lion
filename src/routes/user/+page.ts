import type { PageLoad } from "./$types";

interface HealthCheck {
	ok: boolean;
	message: string;
	data: {
		ping: string;
	};
}

export const load = (async ({ fetch, params, setHeaders, route }) => {
	console.log(`Current Route: ${route.id}`);

	const response = await fetch("http://localhost:8080/public/health");

	if (response.status === 200) {
		const data = (await response.json()) as HealthCheck;

		console.log(data);

		return data;
	} else {
		return {
			ok: false,
			message: "Error fetching health check",
			data: {
				ping: "pong",
			},
		};
	}
}) satisfies PageLoad;
