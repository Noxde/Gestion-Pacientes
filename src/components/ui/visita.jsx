import { Input } from "./input";
import { Calendar28 } from "./dateInput";
import InputLabel from "./inputLabel";
import AreaLabel from "./areaLabel";
import { forwardRef, useEffect, useState } from "react";
import { cn } from "@/lib/utils";

const Visita = forwardRef(function (
  { className, readOnly, visita, onChange },
  ref
) {
  const [visit, setVisit] = useState(
    visita ?? {
      datetime: new Date(),
      reason: "",
      diagnosis: "",
      treatment: "",
      notes: "",
    }
  );

  useEffect(() => {
    if (typeof onChange == "function") {
      onChange(visit);
    }
  }, [visit]);

  return (
    <div
      ref={ref}
      className={cn(
        `flex flex-col gap-2 ${visita ? "pointer-events-none" : ""}`,
        className
      )}
    >
      <div className="flex gap-5 justify-stretch">
        <Calendar28
          onChange={(e) => setVisit((prev) => ({ ...prev, datetime: e }))}
          dateValue={new Date(visit?.datetime)}
          readOnly={readOnly}
          label={"Fecha de Consulta"}
        />
        <InputLabel
          onChange={(e) =>
            setVisit((prev) => ({ ...prev, reason: e.target.value }))
          }
          value={visit?.reason}
          label={"Motivo de Consulta"}
          readOnly={readOnly}
        />
      </div>
      <AreaLabel
        onChange={(e) =>
          setVisit((prev) => ({ ...prev, diagnosis: e.target.value }))
        }
        value={visit?.diagnosis}
        readOnly={readOnly}
        label={"Diagnostico"}
      />
      <AreaLabel
        onChange={(e) =>
          setVisit((prev) => ({ ...prev, treatment: e.target.value }))
        }
        value={visit?.treatment}
        readOnly={readOnly}
        label={"Tratamiento"}
      />
      <AreaLabel
        onChange={(e) =>
          setVisit((prev) => ({ ...prev, notes: e.target.value }))
        }
        readOnly={readOnly}
        value={visit?.notes}
        label={"Notas Adicionales"}
        placeholder="Observaciones, recomendaciones, proximas citas"
      />
    </div>
  );
});

export default Visita;
