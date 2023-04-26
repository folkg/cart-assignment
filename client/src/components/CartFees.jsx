import React from "react";
import "./CartFees.css";

export default function CartFees({ lineItems }) {
  const SUBTOTAL = 2094.97;
  const HST = 272.3461;
  const TOTAL = 2382.3161;
  const SHIPPING = "Free";
  return (
    <>
      <div className="line">
        <div>Subtotal</div>
        <div>${SUBTOTAL.toFixed(2)}</div>
      </div>
      <div className="line">
        <div>Taxes</div>
        <div>${HST.toFixed(2)}</div>
      </div>
      <div className="line">
        <div>Shipping</div>
        <div>${SHIPPING}</div>
      </div>
      <div className="line total">
        <div>Total</div>
        <div>${TOTAL.toFixed(2)}</div>
      </div>
    </>
  );
}
