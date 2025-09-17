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
        {patient.name} {patient.surname}
      </span>
      <span>DNI {patient.national_id}</span>
    </div>
  );
}

export default PatientItem;
