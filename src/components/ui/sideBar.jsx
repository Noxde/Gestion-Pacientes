import { Search, Plus } from "lucide-react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import PatientItem from "@/components/ui/patientItem";
import { useContext, useState } from "react";
import { PatientsContext } from "@/context/patientsContext";

function SideBar({ setAddPatient }) {
  const { patients, setSelected, selected } = useContext(PatientsContext);
  const [search, setSearch] = useState("");

  return (
    <div className="bg-white border flex flex-col pt-5 h-full overflow-hidden">
      {/* Top */}
      <div className="pb-4 px-5">
        {/* Search */}
        <div className="flex items-center bg-gray-50 px-3 rounded-md gap-2 text-gray-700 border">
          <Search size="20px" />
          <Input
            value={search}
            placeholder="Buscar"
            type="text"
            className=" bg-transparent border-0 p-0 shadow-none focus-visible:ring-0"
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>
        <Button
          onClick={() => {
            setAddPatient(true);
            setSelected(null);
          }}
          className="mt-2 w-full cursor-pointer"
        >
          <Plus className="text-gray-50" />
          Agregar Paciente
        </Button>
      </div>

      {/* Patient list */}
      <div className="patient-list flex flex-col h-full min-h-0">
        <h3 className="font-bold text-xl px-5 pb-2 border-b">Pacientes</h3>
        <div className="patients flex-1 overflow-y-auto ">
          {search.trim()
            ? patients
                .filter(
                  (x) =>
                    x.name.toLowerCase().includes(search) ||
                    x.surname.toLowerCase().includes(search) ||
                    x.national_id.includes(search)
                )
                .map((p, i) => (
                  <PatientItem
                    patient={p}
                    className={
                      selected?.national_id === p.national_id
                        ? "bg-blue-50"
                        : ""
                    }
                    onClick={() => {
                      setSelected(p);
                      setAddPatient(false);
                    }}
                  />
                ))
            : patients.map((p, i) => (
                <PatientItem
                  patient={p}
                  className={
                    selected?.national_id === p.national_id ? "bg-blue-50" : ""
                  }
                  onClick={() => {
                    setSelected(p);
                    setAddPatient(false);
                  }}
                />
              ))}
        </div>
      </div>
    </div>
  );
}

export default SideBar;
