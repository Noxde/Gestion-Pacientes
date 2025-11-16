import React, { useContext } from "react";
import { PatientsContext } from "@/context/patientsContext";
import PatientForm from "./patientForm";
import { invoke } from "@tauri-apps/api/core";
import { AlertContext } from "@/context/alertContext";

function AddPatient({ setAddPatient }) {
  const { setPatients, setSelected } = useContext(PatientsContext);
  /**
   * @type {ContextValue}
   */
  const { showToast } = useContext(AlertContext);

  return (
    <div className="p-5 h-full rounded-md">
      <h2 className="text-xl text-gray-700 mb-5">Agregar Paciente</h2>

      <div className="border p-5 rounded-sm">
        <PatientForm
          header="Agregar Paciente"
          confirmLabel="Guardar Paciente"
          callback={async (form) => {
            try {
              const r = await invoke("save_patient_comm", {
                newPatient: form,
              });
              setPatients((prev) => [...prev, r]);
              setSelected(r);
              setAddPatient(false);
              showToast("Paciente Agregado", "", null, "success", 5000);
            } catch (error) {
              console.log(error);
              if (error.includes("National")) {
                showToast(
                  "Ocurrio un error al agregar al paciente",
                  `Un paciente con el DNI ${form.national_id} ya se encuentra registrado`,
                  null,
                  "error"
                );
              }
            }
          }}
        />
      </div>
    </div>
  );
}

export default AddPatient;
