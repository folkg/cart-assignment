import React from "react";
import "./LineItem.css";

export default function LineItem({ item }) {
  return (
    <div className="container">
      <div className="child">
        <img src={item.image} alt={item.title} />
      </div>
      <div className="child line-item__description">
        <div className="item-title">
          {item.swatchTitle.toUpperCase()} / {item.title} / {item.id}
        </div>
        <div className="item-swatch">
          <div
            className="swatch"
            style={{ backgroundColor: item.swatchColor }}
          ></div>
          <div>{item.swatchTitle}</div>
        </div>
      </div>
      <div className="child line-item__details">
        ${item.price * item.quantity}
      </div>
    </div>
  );
}
