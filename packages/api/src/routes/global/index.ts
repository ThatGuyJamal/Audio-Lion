import express from "express";
import { GetGlobalStatistics } from "../../db/utils.js";

const router = express.Router();

router.get("/stats", async (_req, res) => {
  const data = await GetGlobalStatistics();

  if (!data) {
    return res.status(404).json({
      status: "not found",
      message: "No data found.",
      data: null,
    });
  }

  return res.status(200).json({
    status: "ok",
    message: "Successfully fetched global statistics.",
    data: data,
  });
});

export default router;
