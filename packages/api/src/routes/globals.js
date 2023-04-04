import express from "express";
import { statisticSchema } from "../db.js";
import { GetGlobalStatistics } from "../utils.js";

const router = express.Router();

router.get("/stats", async (req, res) => {
  const data = await GetGlobalStatistics();

  if (!data) {
    return res.status(404).json({
      status: "not found",
      message: "No data found.",
      data: null,
    });
  }

  res.status(200).json({
    status: "ok",
    message: "Successfully fetched global statistics.",
    data: data,
  });
});

export default router;
