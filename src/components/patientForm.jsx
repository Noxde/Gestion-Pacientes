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
import InputLabel from "@/components/ui/inputLabel";

function PatientForm({ value, callback, confirmLabel }) {
  const [form, setForm] = useState(
    value ?? {
      id: 1, // Needed to call the rust function but not used
      name: "",
      surname: "",
      national_id: "",
      phone: "",
      medicare: "",
      medicare_number: "",
      sex: "",
      gender: "",
    }
  );
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
        (key === "sex" && data.sex === "x") ||
        (key === "medicare_number" && data.medicare && !data.medicare_number) ||
        (key === "medicare" && data.medicare_number && !data.medicare)
      ) {
        setErrors((prev) => ({ ...prev, [key]: true }));
        complete = false;
      }
    }

    // If there are no errors
    if (complete && typeof callback === "function") {
      try {
        await callback(form);
      } catch (err) {
        console.log(err);
      }
    }
  }

  return (
    <form onSubmit={handleSave} className="flex flex-col">
      <div className="grid grid-cols-2 grid-rows-4 gap-x-10 gap-y-5">
        <div>
          <InputLabel
            value={form.name}
            className={errors?.name && "ring ring-[#d1242f]"}
            label={"Nombre"}
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
            value={form.surname}
            className={errors?.surname && "ring ring-[#d1242f]"}
            label={"Apellido"}
            name="surname"
            onChange={handleChange}
            placeholder="Ingresa el apellido"
          />
          {errors?.surname && (
            <p className="text-sm text-[#d1242f]">El apellido es obligatorio</p>
          )}
        </div>

        <div>
          <InputLabel
            value={form.national_id}
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
            value={form.phone}
            className={errors?.phone && "ring ring-[#d1242f]"}
            label={"Numero de Telefono"}
            name="phone"
            onChange={handleChange}
            placeholder="Ingresa el telefono"
          />
          {errors?.phone && (
            <p className="text-sm text-[#d1242f]">El telefono es obligatorio</p>
          )}
        </div>

        <div>
          <InputLabel
            value={form.medicare}
            className={errors?.medicare && "ring ring-[#d1242f]"}
            label={"Obra Social (opcional)"}
            name="medicare"
            onChange={(e) => {
              handleChange(e);
              if (!e.target.value.trim()) {
                setForm((prev) => ({ ...prev, medicare: "" }));
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
              La obra social es obligatoria si se especifica un numero
            </p>
          )}
        </div>

        <div>
          <InputLabel
            value={form.medicare_number}
            className={errors?.medicare_number && "ring ring-[#d1242f]"}
            label={"Nro de Obra Social"}
            name="medicare_number"
            onChange={(e) => {
              handleChange(e);
              if (!e.target.value.trim()) {
                setForm((prev) => ({ ...prev, medicare_number: "" }));
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
              El nro de obra social es obligatorio si se especifica una obra
              social
            </p>
          )}
        </div>

        <div>
          <Label className="block text-md text-text-secondary">
            Sexo
            <Select
              value={form.sex}
              name="sex"
              onValueChange={(e) => {
                setForm((prev) => ({ ...prev, sex: e }));
                delete errors.sex;
              }}
            >
              <SelectTrigger
                className={`w-full text-text-primary font-normal ${
                  errors?.sex && "ring ring-[#d1242f]"
                }`}
              >
                <SelectValue placeholder="Selecciona el sexo" />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  {/* Dummy item to have a default value */}
                  <SelectItem className="hidden" value="x"></SelectItem>
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
          value={form.gender}
          label="Genero (opcional)"
          name="gender"
          onChange={(e) => {
            handleChange(e);
            if (!e.target.value.trim()) {
              setForm((prev) => ({ ...prev, gender: "" }));
            }
          }}
          placeholder="Ingresa el genero"
        />
      </div>

      <Separator className="my-7" />

      <div className="self-end">
        <Button>{confirmLabel}</Button>
      </div>
    </form>
  );
}

export default PatientForm;
