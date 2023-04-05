import { Strategy } from "@soerenmetje/passport-discord";
import passport, { Profile } from "passport";
import type { VerifyCallback } from "passport-oauth2";
import { UserSchema } from "../db/schemas.js";
import configs from "../configs";
import refresh from "passport-oauth2-refresh";

interface IProfile extends Profile {
	refreshToken: string;
}

passport.serializeUser((user: any, done) => {
	return done(null, user.id);
});

passport.deserializeUser(async (id: string, done) => {
	try {
		const user = await UserSchema.findById(id);
		return user ? done(null, user) : done(null, null);
	} catch (err) {
		console.log(err);
		return done(err, null);
	}
});

const discordStrategy = new Strategy(
	{
		clientID: configs.discord.clientId,
		clientSecret: configs.discord.clientSecret,
		callbackURL: configs.discord.callbackUrl,
		scope: configs.discord.scopes,
	},
	async (
		accessToken: string,
		refreshToken: string,
		profile: IProfile,
		done: VerifyCallback
	) => {
		const { id: discordId } = profile;
		try {
			profile.refreshToken = refreshToken;
			const existingUser = await UserSchema.findOneAndUpdate(
				{ discordId },
				{ accessToken, refreshToken },
				{ new: true }
			);
			if (existingUser) return done(null, existingUser);
			const newUser = new UserSchema({ discordId, accessToken, refreshToken });
			const savedUser = await newUser.save();
			return done(null, savedUser);
		} catch (err) {
			console.log(err);
			return done(err as any, undefined);
		}
	}
);

passport.use(discordStrategy);
refresh.use(discordStrategy);