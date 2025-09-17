import { useState, useEffect, useContext } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./globals.css";

import SideBar from "@/components/ui/sideBar";
import PatientInfo from "./components/ui/patientInfo";
import AddPatient from "./components/ui/addPatient";
import { PatientsContext } from "./context/patientsContext";

function App() {
  const { selected, setPatients } = useContext(PatientsContext);

  const [addPatient, setAddPatient] = useState(false);

  async function initDb() {
    try {
      await invoke("init_db_comm");
    } catch (err) {
      console.log(err);
    }
  }

  useEffect(() => {
    initDb().then(() => {
      fetchPatients();
    });
  }, []);

  async function fetchPatients() {
    try {
      const result = await invoke("get_patients_comm");
      setPatients(result);
    } catch (err) {
      console.error("Error fetching patients:", err);
    }
  }

  return (
    <main className="h-dvh bg-gray-100 grid grid-cols-[300px_1fr] gap-5">
      <SideBar setAddPatient={setAddPatient} />
      <div className="h-full p-5 min-h-0">
        <div className="border bg-white rounded-md overflow-hidden h-full p-5">
          {!selected && !addPatient ? (
            <div>Selecciona un paciente...</div>
          ) : null}
          {selected && !addPatient ? <PatientInfo selected={selected} /> : null}
          {addPatient && !selected ? (
            <AddPatient setAddPatient={setAddPatient} />
          ) : null}
        </div>
      </div>
    </main>
  );
}

export default App;
