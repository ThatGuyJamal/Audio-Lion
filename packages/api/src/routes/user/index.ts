import express from "express";

const router = express.Router();

router.get("/", async (_req, res) => {
  res.status(200).json({
    status: "ok",
    message: "",
    data: "Not implemented yet.",
  });
});

export default router;
