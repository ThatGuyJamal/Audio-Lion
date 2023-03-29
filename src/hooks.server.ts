import type { HandleServerError } from "@sveltejs/kit";

export const handleError = (({ error, event }: any) => {
  return {
    message: "Whoops!",
    code: error?.code ?? "UNKNOWN",
  };
}) satisfies HandleServerError;

import Hapi from "@hapi/hapi";
import getPort from "get-port";

const init = async () => {
  const toPortOrNotToPort = await getPort({ port: [6969, 9696] });

  const server = Hapi.server({
    port: toPortOrNotToPort,
    host: "localhost",
  });

  await server.start();
  console.log("Server running on %s", server.info.uri);
};

process.on("unhandledRejection", (err) => {
  console.log(err);
  process.exit(1);
});

init();

// make class to handle api requests and responses. Will be used by the svelte store states to manage the data.
