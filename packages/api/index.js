import express from "express";
import config_raw from "./src/config.js";
import { CreateConfig, CreateMongooseConnection } from "./src/utils.js";

// Routes
import globalRoutes from "./src/routes/globals.js";
import userRoutes from "./src/routes/user.js";

// Run setup code before starting server
const config = CreateConfig(config_raw);

CreateMongooseConnection(config);

const app = express();

app.get("/", (req, res) => {
	res.status(200).json({
		message: "Hello World!",
	});
});

app.use("/global", globalRoutes);
app.use("/user", userRoutes)

app.listen(config.port, () => {
	console.log(`Server listening on port ${config.port}`);
});
