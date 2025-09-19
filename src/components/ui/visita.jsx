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
  const [visit, setVisit] = useState({
    fecha: new Date(),
    motivo: "",
    diagnostico: "",
    tratamiento: "",
    notas: "",
  });

  useEffect(() => {
    if (typeof onChange == "function") {
      onChange(visit);
    }
  }, [visit]);

  return (
    <div ref={ref} className={cn("flex flex-col gap-2", className)}>
      <div className="flex gap-5 justify-stretch">
        <Calendar28
          onChange={(e) => setVisit((prev) => ({ ...prev, fecha: e }))}
          dateValue={visita?.fecha}
          readOnly={readOnly}
          label={"Fecha de Consulta"}
        />
        <InputLabel
          onChange={(e) =>
            setVisit((prev) => ({ ...prev, motivo: e.target.value }))
          }
          label={"Motivo de Consulta"}
          value={visita?.motivo}
          readOnly={readOnly}
        />
      </div>
      <AreaLabel
        onChange={(e) =>
          setVisit((prev) => ({ ...prev, diagnostico: e.target.value }))
        }
        value={visita?.diagnostico}
        readOnly={readOnly}
        label={"Diagnostico"}
      />
      <AreaLabel
        onChange={(e) =>
          setVisit((prev) => ({ ...prev, tratamiento: e.target.value }))
        }
        readOnly={readOnly}
        label={"Tratamiento"}
      />
      <AreaLabel
        onChange={(e) =>
          setVisit((prev) => ({ ...prev, notas: e.target.value }))
        }
        readOnly={readOnly}
        label={"Notas Adicionales"}
        placeholder="Observaciones, recomendaciones, proximas citas"
      />
    </div>
  );
});

export default Visita;
