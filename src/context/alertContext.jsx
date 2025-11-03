import { createContext, useState } from "react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";

export const AlertContext = createContext();

export const AlertContextProvider = function ({ children }) {
  /**
   *
   * @param {String} title
   * @param {String} description
   * @param {{label: String, onClick: function}} action
   * @param {"success" | "info" | "warning" | "error"} type
   * @param {Number} duration
   */
  function showToast(title, description, action, type, duration = 3000) {
    const payload = {
      description,
      action,
      duration: 3000,
    };

    switch (type) {
      case "success":
        toast.success(title, payload);
        break;

      case "info":
        toast.info(title, payload);
        break;

      case "warning":
        toast.warning(title, payload);
        break;

      case "error":
        toast.error(title, payload);
        break;

      default:
        break;
    }
  }

  const contextValue = { showToast: showToast };

  return (
    <AlertContext.Provider value={contextValue}>
      {children}
    </AlertContext.Provider>
  );
};
