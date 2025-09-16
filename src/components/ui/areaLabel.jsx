import React from "react";
import { Label } from "./label";
import { Textarea } from "./textarea";

function AreaLabel({ label, placeholder, readOnly, value }) {
  return (
    <div className="flex flex-col gap-1">
      <Label className="text-md">{label}</Label>
      <Textarea readOnly={readOnly} value={value} placeholder={placeholder} />
    </div>
  );
}

export default AreaLabel;
