import React from "react";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";

function AddPatient({ setAddPatient }) {
  return (
    <div className="p-5 h-full rounded-md">
      <h2 className="text-xl text-gray-700 mb-5">Agregar Paciente</h2>
      <div className="border p-5 rounded-sm flex flex-col">
        {/* Left */}
        <div className="grid grid-cols-2 grid-rows-4 gap-x-10 gap-y-5">
          <Label className="block text-md">
            Nombre
            <Input placeholder="Ingresa el nombre" />
          </Label>
          <Label className="block text-md">
            Apellido
            <Input placeholder="Ingresa el apellido" />
          </Label>
          <Label className="block text-md">
            DNI
            <Input placeholder="Ingresa el DNI" />
          </Label>
          <Label className="block text-md">
            Numero de Telefono
            <Input placeholder="Ingresa el telefono" />
          </Label>
          <Label className="block text-md">
            Obra Social
            <Input placeholder="Ingresa la obra social" />
          </Label>
          <Label className="block text-md">
            Nro de Obra Social
            <Input placeholder="Ingresa el nro de obra social" />
          </Label>
          <Label className="block text-md">
            Sexo{" "}
            <Select>
              <SelectTrigger className="w-full">
                <SelectValue placeholder="Selecciona el sexo" />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectItem value="m">M</SelectItem>
                  <SelectItem value="f">F</SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
          </Label>
          <Label className="block text-md">
            Genero
            <Input placeholder="Ingresa el genero" />
          </Label>
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
      </div>
    </div>
  );
}

export default AddPatient;
