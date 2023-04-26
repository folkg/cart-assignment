import React from "react";
import "./LineItem.css";

export default function LineItem({ item }) {
  const ESTIMATED_DELIVERY = "Dec 2 - Dec 15";
  return (
    <div className="container">
      <div className="child">
        <img src={item.image} alt={item.title} />
      </div>
      <div className="child">
        <div className="item-title">
          {item.swatchTitle.toUpperCase()} / {item.title} / {item.quantity}
        </div>
        <div className="item-swatch">
          <div
            className="swatch"
            style={{ backgroundColor: item.swatchColor }}
          ></div>
          <div>{item.swatchTitle}</div>
        </div>
      </div>
      <div className="child item-details">
        <div>${item.price * item.quantity}</div>
        <div>Estimated Delivery Date {ESTIMATED_DELIVERY}</div>
        <button>Remove</button>
      </div>
    </div>
  );
}
