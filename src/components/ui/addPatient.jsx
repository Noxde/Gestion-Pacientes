import React, { useContext, useState } from "react";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/core";
import { PatientsContext } from "@/context/patientsContext";
import InputLabel from "./inputLabel";

function AddPatient({ setAddPatient }) {
  const { setPatients, setSelected } = useContext(PatientsContext);
  const [form, setForm] = useState({
    id: 1, // Needed to call the rust function but not used
  });

  function handleChange(e) {
    const { name, value } = e.target;
    setForm((prev) => ({ ...prev, [name]: value }));
  }

  return (
    <div className="p-5 h-full rounded-md">
      <h2 className="text-xl text-gray-700 mb-5">Agregar Paciente</h2>
      <div className="border p-5 rounded-sm flex flex-col">
        <div className="grid grid-cols-2 grid-rows-4 gap-x-10 gap-y-5">
          <InputLabel
            label={"Nombre"}
            name="name"
            onChange={handleChange}
            placeholder="Ingresa el nombre"
          />
          <InputLabel
            label={"Apellido"}
            name="surname"
            onChange={handleChange}
            placeholder="Ingresa el apellido"
          />
          <InputLabel
            label={"DNI"}
            name="national_id"
            onChange={handleChange}
            placeholder="Ingresa el DNI"
          />
          <InputLabel
            label={"Numero de Telefono"}
            name="phone"
            onChange={handleChange}
            placeholder="Ingresa el telefono"
          />
          <InputLabel
            label={"Obra Social"}
            name="medicare"
            onChange={handleChange}
            placeholder="Ingresa la obra social"
          />
          <InputLabel
            label={"Nro de Obra Social"}
            name="medicare_number"
            onChange={handleChange}
            placeholder="Ingresa el nro de obra social"
          />

          <Label className="block text-md">
            Sexo
            <Select
              name="sex"
              onValueChange={(e) => setForm((prev) => ({ ...prev, sex: e }))}
            >
              <SelectTrigger className="w-full">
                <SelectValue placeholder="Selecciona el sexo" />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectItem value="Male">M</SelectItem>
                  <SelectItem value="Female">F</SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
          </Label>
          <InputLabel
            label="Genero"
            name="gender"
            onChange={handleChange}
            placeholder="Ingresa el genero"
          />
        </div>

        <Separator className="my-7" />

        <div className="self-end">
          <Button
            onClick={() => setAddPatient(false)}
            variant="outline"
            className="mr-5 font-bold"
          >
            Cancelar
          </Button>

          {/* TODO: Check that all inputs are filled before being able to save */}
          <Button
            onClick={async () => {
              try {
                const r = await invoke("save_patient_comm", {
                  newPatient: form,
                });
                setPatients((prev) => [...prev, r]);
                setSelected(r);
                setAddPatient(false);
              } catch (err) {
                console.log(err);
              }
            }}
            className="font-bold"
          >
            Guardar Paciente
          </Button>
        </div>
      </div>
    </div>
  );
}

export default AddPatient;
