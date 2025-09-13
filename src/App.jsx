import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./globals.css";

import SideBar from "@/components/ui/sideBar";
import PatientInfo from "./components/ui/patientInfo";
import AddPatient from "./components/ui/addPatient";

function App() {
  const [patientName, setPatientName] = useState("");
  const [createdPatient, setCreatedPatient] = useState("");
  const [dbMsg, setDbMsg] = useState("");
  const [patients, setPatients] = useState([]); //store patients

  const [selected, setSelected] = useState(null);
  const [addPatient, setAddPatient] = useState(false);
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  async function savePatient() {
    try {
      const patient = await invoke("save_patient_comm", {
        newPatient: { id: 0, name: patientName },
      });
      setCreatedPatient(patient);
    } catch (err) {
      setCreatedPatient({ error: err });
    }
  }

  async function initDb() {
    try {
      await invoke("init_db_comm");
      setDbMsg("Database initalized successfully!");
    } catch (err) {
      setDbMsg(`Error: ${err}`);
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
    <main className="h-dvh bg-gray-100 grid grid-cols-[300px_1fr] min-h-0">
      <SideBar
        setAddPatient={setAddPatient}
        setSelected={setSelected}
        selected={selected}
      />
      <div className="bg-white border rounded-md ml-5 m-2">
        {addPatient ? (
          <AddPatient setAddPatient={setAddPatient} />
        ) : (
          <PatientInfo selected={selected} />
        )}
      </div>
    </main>
  );
}

export default App;
