import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { PatientsContextProvider } from "./context/patientsContext";
import { AlertContextProvider } from "./context/alertContext";

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <AlertContextProvider>
      <PatientsContextProvider>
        <App />
      </PatientsContextProvider>
    </AlertContextProvider>
  </React.StrictMode>
);
