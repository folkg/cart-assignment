import React from "react";
import "./CartFees.css";

export default function CartFees({ fees }) {
  const { subtotal, tax, shipping, total } = fees;
  return (
    <>
      <div className="line">
        <div>Subtotal</div>
        <div>${subtotal.toFixed(2)}</div>
      </div>
      <div className="line">
        <div>Taxes</div>
        <div>${tax.toFixed(2)}</div>
      </div>
      <div className="line">
        <div>Shipping</div>
        <div>${shipping.toFixed(2)}</div>
      </div>
      <div className="line total">
        <div>Total</div>
        <div>${total.toFixed(2)}</div>
      </div>
    </>
  );
}
