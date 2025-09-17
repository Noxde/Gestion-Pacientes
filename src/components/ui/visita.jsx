import { Input } from "./input";
import { Calendar28 } from "./dateInput";
import InputLabel from "./inputLabel";
import AreaLabel from "./areaLabel";
import { forwardRef } from "react";
import { cn } from "@/lib/utils";

const Visita = forwardRef(function ({ className, readOnly, visita }, ref) {
  return (
    <div ref={ref} className={cn("flex flex-col gap-2", className)}>
      <div className="flex gap-5 justify-stretch">
        <Calendar28
          dateValue={visita?.fecha}
          readOnly={readOnly}
          label={"Fecha de Consulta"}
        />
        <InputLabel
          label={"Motivo de Consulta"}
          value={visita?.motivo}
          readOnly={readOnly}
        />
      </div>
      <AreaLabel
        value={visita?.diagnostico}
        readOnly={readOnly}
        label={"Diagnostico"}
      />
      <AreaLabel readOnly={readOnly} label={"Tratamiento"} />
      <AreaLabel
        readOnly={readOnly}
        label={"Notas Adicionales"}
        placeholder="Observaciones, recomendaciones, proximas citas"
      />
    </div>
  );
});

export default Visita;
