import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [patientName, setPatientName] = useState("");
  const [createdPatient, setCreatedPatient] = useState("");
  const [dbMsg, setDbMsg] = useState("");
  const [patients, setPatients] = useState([]); //store patients

  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  async function savePatient() {
    try {
      const patient = await invoke("save_patient_comm", {
        newPatient: { id: 0, name: patientName},
      });
      setCreatedPatient(patient);
    } catch (err) {
      setCreatedPatient({ error: err});
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
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      {/* Patient creation form */}
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          savePatient();
        }}
      >
        <input
          id="patient-input"
          value={patientName}
          onChange={(e) => setPatientName(e.currentTarget.value)}
          placeholder="Enter patient name..."
        />
        <button type="submit">Save Patient</button>
      </form>

      {/* Show created patient */}
      {createdPatient && !createdPatient.error && (
        <p>
          Patient created &gt; Name: {createdPatient.name}, ID: {createdPatient.id}
        </p>
      )}
      {createdPatient?.error && (
        <p style={{ color: "red" }}>Error: {createdPatient.error}</p>
      )}

      <div className="row">
        <button onClick={initDb}>Initialize Database</button>
        <button onClick={fetchPatients}>Load Patients</button>
      </div>
      <p>{dbMsg}</p>

      {/* Patient list */}
      {patients.length > 0 && (
        <div>
          <h2>Patients</h2>
          <ul>
            {patients.map((p) => (
              <li key={p.id}>
                {p.id} - {p.name}
              </li>
            ))}
          </ul>
        </div>
      )}

    </main>
  );
}

export default App;
