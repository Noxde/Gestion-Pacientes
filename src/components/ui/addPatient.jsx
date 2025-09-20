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
    name: "",
    surname: "",
    national_id: "",
    phone: "",
    medicare: "-",
    medicare_number: "-",
    sex: "",
    gender: "-",
  });
  const [errors, setErrors] = useState({});

  function handleChange(e) {
    const { name, value } = e.target;
    setForm((prev) => ({ ...prev, [name]: value }));
    setErrors((prev) => {
      delete prev[name];
      return prev;
    });
  }

  async function handleSave(e) {
    e.preventDefault();
    const { target } = e;
    const formData = new FormData(target);
    const data = Object.fromEntries(formData.entries());

    // Set errors
    let complete = true;
    for (const key in data) {
      if (
        (key !== "medicare" &&
          key !== "medicare_number" &&
          key !== "gender" &&
          !data[key]) ||
        (key === "medicare_number" && data.medicare && !data.medicare_number) ||
        (key === "medicare" && data.medicare_number && !data.medicare)
      ) {
        setErrors((prev) => ({ ...prev, [key]: true }));
        complete = false;
      }
    }

    // If there are no errors
    if (complete) {
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
    }
  }

  return (
    <div className="p-5 h-full rounded-md">
      <h2 className="text-xl text-gray-700 mb-5">Agregar Paciente</h2>
      <form
        className="border p-5 rounded-sm flex flex-col"
        onSubmit={handleSave}
      >
        <div className="grid grid-cols-2 grid-rows-4 gap-x-10 gap-y-5">
          <div>
            <InputLabel
              className={errors?.name && "ring ring-[#d1242f]"}
              label={"Nombre*"}
              name="name"
              onChange={handleChange}
              placeholder="Ingresa el nombre"
            />
            {errors?.name && (
              <p className="text-sm text-[#d1242f]">El nombre es obligatorio</p>
            )}
          </div>

          <div>
            <InputLabel
              className={errors?.surname && "ring ring-[#d1242f]"}
              label={"Apellido"}
              name="surname"
              onChange={handleChange}
              placeholder="Ingresa el apellido"
            />
            {errors?.surname && (
              <p className="text-sm text-[#d1242f]">
                El apellido es obligatorio
              </p>
            )}
          </div>

          <div>
            <InputLabel
              className={errors?.national_id && "ring ring-[#d1242f]"}
              label={"DNI"}
              name="national_id"
              onChange={handleChange}
              placeholder="Ingresa el DNI"
            />
            {errors?.national_id && (
              <p className="text-sm text-[#d1242f]">El DNI es obligatorio</p>
            )}
          </div>

          <div>
            <InputLabel
              className={errors?.phone && "ring ring-[#d1242f]"}
              label={"Numero de Telefono"}
              name="phone"
              onChange={handleChange}
              placeholder="Ingresa el telefono"
            />
            {errors?.phone && (
              <p className="text-sm text-[#d1242f]">
                El telefono es obligatorio
              </p>
            )}
          </div>

          <div>
            <InputLabel
              className={errors?.medicare && "ring ring-[#d1242f]"}
              label={"Obra Social"}
              name="medicare"
              onChange={(e) => {
                handleChange(e);
                if (!e.target.value.trim()) {
                  setForm((prev) => ({ ...prev, medicare: "-" }));
                  setErrors((prev) => {
                    delete prev.medicare_number;
                    return prev;
                  });
                }
              }}
              placeholder="Ingresa la obra social"
            />
            {errors?.medicare && (
              <p className="text-sm text-[#d1242f]">
                La obra social es obligatoria
              </p>
            )}
          </div>

          <div>
            <InputLabel
              className={errors?.medicare_number && "ring ring-[#d1242f]"}
              label={"Nro de Obra Social"}
              name="medicare_number"
              onChange={(e) => {
                handleChange(e);
                if (!e.target.value.trim()) {
                  setForm((prev) => ({ ...prev, medicare_number: "-" }));
                  setErrors((prev) => {
                    delete prev.medicare;
                    return prev;
                  });
                }
              }}
              placeholder="Ingresa el nro de obra social"
            />
            {errors?.medicare_number && (
              <p className="text-sm text-[#d1242f]">
                El nro de obra social es obligatorio
              </p>
            )}
          </div>

          <div>
            <Label className="block text-md">
              Sexo
              <Select
                name="sex"
                onValueChange={(e) => {
                  setForm((prev) => ({ ...prev, sex: e }));
                  delete errors.sex;
                }}
              >
                <SelectTrigger
                  className={`w-full ${errors?.sex && "ring ring-[#d1242f]"}`}
                >
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
            {errors?.sex && (
              <p className="text-sm text-[#d1242f]">El sexo es obligatorio</p>
            )}
          </div>
          <InputLabel
            label="Genero"
            name="gender"
            onChange={(e) => {
              handleChange(e);
              if (!e.target.value.trim()) {
                setForm((prev) => ({ ...prev, gender: "-" }));
              }
            }}
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

          <Button className="font-bold">Guardar Paciente</Button>
        </div>
      </form>
    </div>
  );
}

export default AddPatient;
