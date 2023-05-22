import { Router } from "express";

const router = Router();

const DELIVERY_DATES = [
  {
    postal: "V",
    ids: [2],
    estimatedDeliveryDate: "Nov 24, 2021",
  },
  {
    postal: "V",
    ids: [1, 3],
    estimatedDeliveryDate: "Nov 19, 2021",
  },
  {
    postal: "M",
    ids: [2, 3],
    estimatedDeliveryDate: "Nov 22, 2021",
  },
  {
    postal: "M",
    ids: [1],
    estimatedDeliveryDate: "Dec 19, 2021",
  },
  {
    postal: "K",
    ids: [1, 2, 3],
    estimatedDeliveryDate: "Dec 24, 2021",
  },
];

router.get("/", (req, res) => {
  // will return the first found delivery date if multiple item ids match
  const postalCode = req.query.postalCode;
  const lineItemId = req.query.lineItemId;
  const deliveryDate = DELIVERY_DATES.find((deliveryDate) => {
    return (
      deliveryDate.postal === postalCode?.[0] &&
      deliveryDate.ids.includes(parseInt(lineItemId))
    );
  });
  res.json(deliveryDate?.estimatedDeliveryDate || "TBD");
});

export default router;
