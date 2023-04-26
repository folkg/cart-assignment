import React, { useState, useEffect } from "react";
import "./LineItem.css";

export default function LineItem({ item, removeLineItemFn, postalCode }) {
  const [deliveryDate, setDeliveryDate] = useState("TBD");
  useEffect(() => {
    fetch(
      `http://localhost:4000/api/delivery?postalCode=${postalCode}&lineItemId=${item.id}`
    )
      .then((res) => res.json())
      .then((data) => {
        setDeliveryDate(data);
      });
  }, [postalCode]);
  function handleClick() {
    removeLineItemFn(item.id);
  }
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
        <div>Estimated Delivery Date {deliveryDate}</div>
        <button onClick={handleClick}>Remove</button>
      </div>
    </div>
  );
}
