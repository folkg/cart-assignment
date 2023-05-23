import { Router } from "express";
import { readFileSync } from "fs";

const router = Router();

router.get("/", (req, res) => {
  // will return the first found delivery date if multiple item ids match
  let delivery_dates;
  try {
    const fileData = readFileSync("./json/delivery_dates.json", "utf8");
    delivery_dates = JSON.parse(fileData);
  } catch (err) {
    console.error("Error reading file:", err);
  }
  const postalCode = req.query.postalCode;
  const lineItemId = req.query.lineItemId;
  const deliveryDate = delivery_dates.find((deliveryDate) => {
    return (
      deliveryDate.postal === postalCode?.[0] &&
      deliveryDate.ids.includes(parseInt(lineItemId))
    );
  });
  res.json(deliveryDate?.estimatedDeliveryDate || "TBD");
});

export default router;
