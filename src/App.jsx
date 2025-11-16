import { useState, useEffect, useContext } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./globals.css";

import SideBar from "@/components/ui/Sidebar/sideBar";
import PatientInfo from "./components/ui/patientInfo";
import AddPatient from "./components/ui/addPatient";
import { PatientsContext } from "./context/patientsContext";
import { UserSearch } from "lucide-react";
import { AlertContext } from "./context/alertContext";
import { Toaster } from "./components/shadcn/sonner";

function App() {
  const { selected, setPatients, patients } = useContext(PatientsContext);
  const { showToast } = useContext(AlertContext);

  const [addPatient, setAddPatient] = useState(false);

  async function initDb() {
    try {
      await invoke("init_db_comm");
    } catch (err) {
      console.log(err);
    }
  }

  useEffect(() => {
    function initDatabase() {
      initDb()
        .then(() => {
          fetchPatients();
        })
        .catch((err) => {
          showToast(
            "Hubo un error al inicializar la base de datos",
            "",
            { label: "Reintentar", onClick: initDatabase },
            "error"
          );
        });
    }

    initDatabase();
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
    <>
      <main className="h-dvh bg-gray-100 grid grid-cols-[300px_1fr] gap-5">
        <SideBar setAddPatient={setAddPatient} />
        <div className="h-full p-5 min-h-0">
          <div className="border bg-white rounded-md overflow-hidden h-full p-5">
            {!selected && !addPatient ? (
              <div className="h-full flex flex-col items-center justify-center">
                <UserSearch size="50px" className="text-text-secondary" />
                <p className="text-text-primary font-medium text-lg">
                  Seleccione un paciente
                </p>
                <p className="text-text-secondary max-w-md text-center">
                  Seleccione un paciente de la lista para ver sus detalles o
                  agregue un paciente nuevo haciendo click en "Agregar Paciente"
                </p>
              </div>
            ) : null}
            {selected && !addPatient ? (
              <PatientInfo key={selected.id} selected={selected} />
            ) : null}
            {addPatient && !selected ? (
              <AddPatient setAddPatient={setAddPatient} />
            ) : null}
          </div>
        </div>
      </main>
      <Toaster richColors theme="light" />
    </>
  );
}

export default App;
