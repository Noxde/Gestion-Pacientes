import React, { useContext } from "react";
import { PatientsContext } from "@/context/patientsContext";
import PatientForm from "../patientForm";
import { invoke } from "@tauri-apps/api/core";

function AddPatient({ setAddPatient }) {
  const { setPatients, setSelected } = useContext(PatientsContext);

  return (
    <div className="p-5 h-full rounded-md">
      <h2 className="text-xl text-gray-700 mb-5">Agregar Paciente</h2>

      <div className="border p-5 rounded-sm">
        <PatientForm
          header="Agregar Paciente"
          confirmLabel="Guardar Paciente"
          callback={async (form) => {
            const r = await invoke("save_patient_comm", {
              newPatient: form,
            });
            setPatients((prev) => [...prev, r]);
            setSelected(r);
            setAddPatient(false);
          }}
        />
      </div>
    </div>
  );
}

export default AddPatient;
