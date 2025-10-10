import React from "react";

function PatientItem({ patient, onClick, selected }) {
  return (
    <div
      onClick={onClick}
      className={`flex flex-col hover:bg-blue-50 px-5 py-1 cursor-pointer ${
        selected ? "bg-[#dbeafe] border-l-5 border-[#607afb]" : ""
      }`}
    >
      <span className={`font-bold ${selected ? "text-[#607afb]" : ""}`}>
        {patient.name} {patient.surname}
      </span>
      <span className="text-text-secondary">DNI {patient.national_id}</span>
    </div>
  );
}

export default PatientItem;
