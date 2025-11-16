import React from "react";
import { Label } from "../shadcn/label";
import { Textarea } from "../shadcn/textarea";

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
