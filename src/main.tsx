import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import List from "@/pages/list";
import ErrorPage from "@/pages/error-page";
import Settings from "@/pages/settings";
import "./index.css";


const router = createBrowserRouter([
  {
    path: "/",
    Component: App,
    errorElement: <ErrorPage />,
    children: [
      {
        path: "",
        Component: List,
      },
      {
        path: "settings",
        Component: Settings,
      },
    ],

  },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <RouterProvider router={router} />
);
