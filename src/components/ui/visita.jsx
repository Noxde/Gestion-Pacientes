import { Calendar28 } from "./dateInput";
import InputLabel from "./inputLabel";
import AreaLabel from "./areaLabel";
import { forwardRef, useEffect, useState } from "react";
import { cn } from "@/lib/utils";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import ImageUpload from "./imageUpload";
import FilesGallery from "./filesGallery";

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
      docs: [],
    }
  );
  const [files, setFiles] = useState([]);

  useEffect(() => {
    if (typeof onChange == "function") {
      onChange(visit);
    }
  }, [visit]);

  useEffect(() => {
    setVisit((prev) => ({ ...prev, docs: files }));
    console.log(files);
  }, [files]);

  return (
    <Tabs defaultValue="detalles">
      <TabsList className="rounded-none border-b p-0 w-full">
        <TabsTrigger
          className="bg-background data-[state=active]:border-primary data-[state=active]:shadow-none h-full rounded-none border-b-2 border-x-0 border-t-0 border-transparent"
          value="detalles"
        >
          Detalles de la visita
        </TabsTrigger>
        <TabsTrigger
          className="bg-background data-[state=active]:border-primary data-[state=active]:shadow-none h-full rounded-none border-b-2 border-x-0 border-t-0 border-transparent"
          value="adjuntos"
        >
          Adjuntos {visita ? `(${visita.docs.length})` : ""}
        </TabsTrigger>
      </TabsList>
      <TabsContent value="detalles">
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
      </TabsContent>
      <TabsContent value="adjuntos">
        {readOnly ? (
          <FilesGallery files={visita.docs} />
        ) : (
          <ImageUpload setVisit={setVisit} setFiles={setFiles} files={files} />
        )}
      </TabsContent>
    </Tabs>
  );
});

export default Visita;
