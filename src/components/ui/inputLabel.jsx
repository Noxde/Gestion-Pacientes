import React, { useState } from "react";
import { Label } from "../shadcn/label";
import { Input } from "../shadcn/input";

function InputLabel({
  className,
  label,
  placeholder,
  name,
  readOnly,
  value,
  onChange,
}) {
  return (
    <div className="flex-1 flex flex-col gap-1">
      <Label className="text-md text-text-secondary">{label}</Label>
      <Input
        className="bg-white"
        name={name}
        onChange={onChange}
        placeholder={placeholder}
        value={value}
        readOnly={readOnly}
      />
    </div>
  );
}

export default InputLabel;
