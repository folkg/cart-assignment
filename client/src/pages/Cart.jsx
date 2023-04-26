import React from "react";
import "./Cart.css";
import LineItem from "../components/LineItem";
import CartFees from "../components/CartFees";

export default function Cart({
  lineItems,
  removeLineItemFn,
  postalCode,
  setPostalCode,
}) {
  const SHIPPING = 15;
  const calculateFees = () => {
    // Assume that shipping is not taxed
    console.log("fees calculated");
    const subtotal = lineItems.reduce((acc, item) => {
      return acc + item.price * item.quantity;
    }, 0);
    const tax = subtotal * 0.13;
    const shipping = tax > 0 ? SHIPPING : 0;
    const total = subtotal + tax + shipping;
    return { subtotal, tax, shipping, total };
  };
  return (
    <div>
      <h1>Your Cart</h1>
      {lineItems.map((item) => (
        <div className="line-item" key={item.id}>
          <LineItem
            item={item}
            removeLineItemFn={removeLineItemFn}
            postalCode={postalCode}
          />
        </div>
      ))}
      <CartFees fees={calculateFees()} />
      <div>Postal Code</div>
      <input
        type="text"
        value={postalCode}
        onChange={(e) => setPostalCode(e.target.value)}
      />
    </div>
  );
}
