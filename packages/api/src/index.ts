// Lib imports
import express from "express";
import cors from "cors";
import session from "express-session";
import passport from "passport";
import store from "connect-mongo";
import helmet from "helmet";
import { config as LoadEnv } from "dotenv";

LoadEnv(); // Load environment variables from the .env file

// Local imports
import routes from "./routes/index.js";
import config_raw, { HostUrl } from "./configs/index.js";
import { CreateConfig } from "./utils/index.js";
import { errorMiddleware, loggerMiddleware } from "./middleware/utils.js";
import { CreateMongooseConnection } from "./db/utils.js";

require("./strategies/discord.js"); // Starting the Discord passport strategy

const app = express();
const config = CreateConfig(config_raw);
CreateMongooseConnection(config);

// Middleware
app.use(loggerMiddleware);
app.use(errorMiddleware);
app.use(helmet());
// Enable Parsing Middleware for Requests
app.use(express.json());
app.use(express.urlencoded({ extended: true }));
// Enable CORS
app.use(cors({ origin: [HostUrl], credentials: true }));
// Enable Sessions
app.use(
	session({
		secret: config.sessionSecret,
		resave: false,
		saveUninitialized: false,
		cookie: { maxAge: 60000 * 60 * 24 * 7 }, // 7 days
		store: store.create({
			mongoUrl: config.mongodbUrl,
			autoRemove: "native", // Default
		}),
	})
);
// app.use((_req, _res, next) => setTimeout(() => next(), 100));
// Enable Passport
app.use(passport.initialize());
app.use(passport.session());

// Routes
app.use("/api", routes);

app.listen(config.port, () => {
	console.log(`Running in ${process.env.NODE_ENV} mode.`);
	console.log(`Server listening at ${HostUrl}`);
});
