import { cn } from "@/lib/utils";
import React from "react";

function PatientItem({ onClick, className }) {
  return (
    <div
      onClick={onClick}
      className={cn(
        "flex flex-col hover:bg-blue-50 px-5 py-1 cursor-pointer",
        className
      )}
    >
      <span className="font-bold">Nombre Apellido</span>
      <span>DNI 123456789</span>
    </div>
  );
}

export default PatientItem;
