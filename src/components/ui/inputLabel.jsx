import React, { useState } from "react";
import { Label } from "./label";
import { Input } from "./input";

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
      <Label className="text-md">{label}</Label>
      <Input
        className={className}
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
