import React from "react";
import "./Cart.css";

export default function Cart({ lineItems }) {
  return (
    <div>
      <h1>Your Cart</h1>
      {lineItems.map((item) => (
        <div className="line-item" key={item.id}>
          {item.title}
        </div>
      ))}
    </div>
  );
}
