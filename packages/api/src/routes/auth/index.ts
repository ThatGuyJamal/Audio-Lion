import express from "express";
import passport from "passport";
import configs from "../../configs";

const router = express.Router();

router.get("/discord", passport.authenticate("discord")).get(
  "/discord/callback",
  passport.authenticate("discord", {
    failureRedirect: "/",
    successRedirect: `${configs.discord.authSuccessUrl}/profile`,
  })
);

export default router;