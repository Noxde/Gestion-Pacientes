import {
  ArrowDown,
  ArrowUp,
  ClipboardList,
  FileText,
  Folder,
} from "lucide-react";
import { Button } from "./button";
import { Separator } from "./separator";
import { useEffect, useState, useRef, useContext } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import Visita from "./visita";
import PatientForm from "../patientForm";
import { invoke } from "@tauri-apps/api/core";
import { PatientsContext } from "@/context/patientsContext";

function PatientInfo({ selected }) {
  const [showing, setShowing] = useState(0);
  const refs = useRef([]);
  const [toAdd, setToAdd] = useState({});
  const [visitas, setVisitas] = useState([]);
  const [dialogContent, setDialogContent] = useState("add");
  const { setPatients, setSelected } = useContext(PatientsContext);
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  useEffect(() => {
    refs.current[showing]?.scrollIntoView({
      behavior: "smooth",
      scrollMarginTop: "20px",
    });
  }, [showing]);

  // Get visits on mount
  useEffect(() => {
    invoke("get_events_comm", {
      patientId: selected.id,
    })
      .then((r) => {
        setVisitas(
          r.map((x) => ({
            fecha: new Date(x.datetime),
            motivo: x.title,
            diagnostico: x.description,
            tratamiento: "",
            notas: "",
          }))
        );
      })
      .catch((err) => {
        // No visits
        console.error(err);
      });
  }, []);

  return (
    <Dialog open={isDialogOpen}>
      <DialogContent
        showCloseButton={false}
        onInteractOutside={() => setIsDialogOpen(false)}
      >
        {dialogContent === "add" ? (
          <>
            <DialogHeader>
              <DialogTitle>Agregar Visita</DialogTitle>
            </DialogHeader>

            <Separator />
            <Visita
              onChange={(e) => {
                console.log(e);
                setToAdd(e);
              }}
            />
            <Separator />

            <div className="flex gap-2 justify-self-end">
              <Button
                onClick={() => setIsDialogOpen(false)}
                variant={"outline"}
              >
                Cancelar
              </Button>
              <Button
                onClick={async () => {
                  try {
                    const res = await invoke("save_event_comm", {
                      newEvent: {
                        id: 1,
                        patient_id: selected.id,
                        title: toAdd.motivo,
                        description: toAdd.diagnostico,
                        datetime: new Date(
                          toAdd.fecha.getTime() -
                            toAdd.fecha.getTimezoneOffset() * 60000 // Fix for UTC date
                        )
                          .toJSON()
                          .replace("Z", ""),
                      },
                      patientId: selected.id,
                    });
                    setVisitas((prev) => [{ ...toAdd, id: res.id }, ...prev]);
                    setToAdd({});
                    setIsDialogOpen(false);
                  } catch (err) {
                    console.error(err);
                  }
                }}
              >
                Agregar
              </Button>
            </div>
          </>
        ) : (
          <>
            <DialogHeader>
              <DialogTitle>Editar Paciente</DialogTitle>
            </DialogHeader>

            <Separator />
            <PatientForm
              header="Editar Paciente"
              confirmLabel="Guardar Cambios"
              value={selected}
              callback={async (form) => {
                await invoke("update_patient_comm", {
                  patient: form,
                });
                setSelected(form);
                setPatients((prev) =>
                  prev.map((p, i) => (form.id === p.id ? form : p))
                );
                setIsDialogOpen(false);
              }}
            />
          </>
        )}
      </DialogContent>

      <div className=" flex flex-col h-full">
        {/* Info */}
        <div className="flex items-center justify-between">
          <span className="">
            {selected?.name} {selected?.surname} / {selected?.medicare_number} /{" "}
            {selected?.national_id}
          </span>

          <Button
            className="cursor-pointer"
            variant="outline"
            onClick={() => {
              setDialogContent("edit");
              setIsDialogOpen(true);
            }}
          >
            Editar
          </Button>
        </div>

        <Separator className="my-5" />
        {/* Visits */}
        <div className="flex items-center justify-between mb-5">
          <span className="flex items-center gap-2 text-xl">
            <ClipboardList className="text-text-primary" /> Historial de Visitas
            ({visitas.length})
          </span>

          <Button
            onClick={() => {
              setDialogContent("add");
              setIsDialogOpen(true);
            }}
          >
            Agregar Visita
          </Button>
        </div>

        {/* scroll */}
        <div className="h-full grid grid-cols-[1fr_50px] min-h-0 relative rounded-md border">
          <div className="flex flex-col px-20 overflow-auto">
            {/* Show when there are no visits */}
            {!visitas.length ? (
              <div className="h-full flex justify-center items-center">
                <div className="flex flex-col items-center justify-center text-center text-muted-foreground h-full">
                  <ClipboardList className="w-12 h-12 mb-3 text-blue-400" />
                  <p className="text-lg font-medium text-text-primary">
                    No hay visitas registradas
                  </p>
                  <p className="text-sm mb-4">
                    Hace click en "Agregar Visita" para añadir la primera
                    consulta.
                  </p>
                </div>
              </div>
            ) : null}
            {visitas.map((x, i, arr) => (
              <>
                <Visita
                  readOnly
                  key={x.id}
                  className="py-5"
                  visita={x}
                  ref={(el) => (refs.current[i] = el)}
                />
                {i < arr.length - 1 ? <Separator /> : null}
              </>
            ))}
          </div>

          <div className="self-center flex flex-col gap-1">
            <Button
              className="h-[75px]"
              onClick={() =>
                setShowing((prev) => {
                  if (prev - 1 < 0) {
                    return visitas.length - 1;
                  } else {
                    return prev - 1;
                  }
                })
              }
            >
              <ArrowUp />
            </Button>
            <Button
              className="h-[75px]"
              onClick={() =>
                setShowing((prev) => {
                  if (prev + 1 > visitas.length - 1) {
                    return 0;
                  } else {
                    return prev + 1;
                  }
                })
              }
            >
              <ArrowDown />
            </Button>
          </div>
        </div>
      </div>
    </Dialog>
  );
}

export default PatientInfo;
