import { cn } from "@/lib/utils";
import React from "react";

function PatientItem({ patient, onClick, className }) {
  return (
    <div
      onClick={onClick}
      className={cn(
        "flex flex-col hover:bg-blue-50 px-5 py-1 cursor-pointer",
        className
      )}
    >
      <span className="font-bold">
        {patient.nombre} {patient.apellido}
      </span>
      <span>DNI {patient.DNI}</span>
    </div>
  );
}

export default PatientItem;
