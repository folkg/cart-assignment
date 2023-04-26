import React from "react";
import "./Cart.css";
import LineItem from "../components/LineItem";
import CartFees from "../components/CartFees";

export default function Cart({ lineItems, removeLineItemFn }) {
  return (
    <div>
      <h1>Your Cart</h1>
      {lineItems.map((item) => (
        <div className="line-item" key={item.id}>
          <LineItem item={item} removeLineItemFn={removeLineItemFn} />
        </div>
      ))}
      <CartFees lineItems={lineItems} />
    </div>
  );
}
