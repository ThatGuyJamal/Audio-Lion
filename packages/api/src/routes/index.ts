import { Router } from "express";

import authRouter from "./auth/index.js";
import globalRouter from "./global/index.js";

const router = Router();

router.use("/auth", authRouter);
router.use("/global", globalRouter);

export default router;