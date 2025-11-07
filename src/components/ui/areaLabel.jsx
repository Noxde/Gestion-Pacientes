import React from "react";
import { Label } from "./label";
import { Textarea } from "./textarea";

function AreaLabel({ label, placeholder, readOnly, value, onChange }) {
  return (
    <div className="flex flex-col gap-1">
      <Label className="text-md text-text-secondary">{label}</Label>
      <Textarea
        className="bg-white"
        onChange={onChange}
        readOnly={readOnly}
        value={value}
        placeholder={placeholder}
      />
    </div>
  );
}

export default AreaLabel;
