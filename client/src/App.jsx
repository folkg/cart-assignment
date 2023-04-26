import "./App.css";
import Cart from "./pages/Cart";
import React, { useEffect, useState } from "react";

function App() {
  //Styling variables
  const BLUE = "#172162"; //"rgb(23, 33, 98)";
  const LIGHT_GREY = "#6e7484";
  const BLACK = "#000000";

  //First part given
  const [lineItems, setLineItems] = useState([]);

  useEffect(() => {
    fetch("http://localhost:4000/api/item")
      .then((res) => res.json())
      .then((data) => {
        setLineItems(data);
      });
  }, []);

  const removeLineItem = (id) => {
    setLineItems((lineItems) => lineItems.filter((item) => item.id !== id));
  };

  const sampleLineItems = [
    {
      id: 1,
      title: "Grey Sofa",
      price: 499.99,
      quantity: 1,
      image:
        "https://www.cozey.ca/_next/image?url=https%3A%2F%2Fcdn.shopify.com%2Fs%2Ffiles%2F1%2F0277%2F3057%2F5462%2Fproducts%2F2_Single_shot_DARK_GREY_OFF_OFF_SLOPE_17f0f115-11f8-4a78-b412-e9a2fea4748d.png%3Fv%3D1629310667&w=1920&q=75",
      swatchColor: "#959392",
      swatchTitle: "Grey",
    },
    {
      id: 2,

      title: "Blue Sofa",
      price: 994.99,
      quantity: 1,
      image:
        "https://www.cozey.ca/_next/image?url=https%3A%2F%2Fcdn.shopify.com%2Fs%2Ffiles%2F1%2F0277%2F3057%2F5462%2Fproducts%2F3_Seater_SofaSofa_Ottoman_Off_Arm_Configuration_Two_Arms_Arm_Design_Slope_Chaise_Off_Fabric_Navy_Blue2.png%3Fv%3D1629231450&w=1920&q=75",
      swatchColor: "#191944",
      swatchTitle: "Blue",
    },
    {
      id: 3,
      title: "White Sofa",
      price: 599.99,
      quantity: 1,

      image:
        "https://www.cozey.ca/_next/image?url=https%3A%2F%2Fcdn.shopify.com%2Fs%2Ffiles%2F1%2F0277%2F3057%2F5462%2Fproducts%2F2_Single_shot_IVORY_OFF_OFF_SLOPE_5379af1f-9318-4e37-b514-962d33d1ce64.png%3Fv%3D1629231450&w=1920&q=75",
      swatchColor: "#F8F1EC",
      swatchTitle: "White",
    },
  ];
  const addLineItem = () => {
    // I was a little confused by the instructions here
    // I chose to add a random item from the default list
    const newItem =
      sampleLineItems[Math.floor(Math.random() * sampleLineItems.length)];
    newItem.id = Math.random() * 1000; // I know this is not a good way to generate unique ids, but quick and dirty
    setLineItems((lineItems) => [...lineItems, newItem]);
  };
  const ESTIMATED_DELIVERY = "Nov 24, 2021";

  return (
    <>
      <Cart lineItems={lineItems} removeLineItemFn={removeLineItem} />
      <button onClick={addLineItem}>Add Item to Cart</button>
    </>
  );
}

export default App;
