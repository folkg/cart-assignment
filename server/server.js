import express from "express";
import cors from "cors";
import itemRouter from "./routes/itemRoutes.js";
import deliveryRouter from "./routes/deliveryRoutes.js";

const server = express();
const hostname = "127.0.0.1";
const port = 4000;

server.use(express.json());
server.use(cors());

server.use("/api/item", itemRouter);
server.use("/api/delivery", deliveryRouter);

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
