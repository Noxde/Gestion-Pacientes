import React from "react";
import { Label } from "./label";
import { Input } from "./input";

function InputLabel({ readOnly, value }) {
  return (
    <div className="flex-1 flex flex-col gap-1">
      <Label className="text-md">Motivo de Consulta</Label>
      <Input value={value} readOnly={readOnly} />
    </div>
  );
}

export default InputLabel;
