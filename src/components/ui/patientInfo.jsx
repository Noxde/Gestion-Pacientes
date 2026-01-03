import { ClipboardList } from "lucide-react";
import { Button } from "../shadcn/button";
import { Separator } from "../shadcn/separator";
import { useEffect, useState, useContext } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/shadcn/dialog";
import Visita from "./Visit/visita";
import PatientForm from "./patientForm";
import { invoke } from "@tauri-apps/api/core";
import { PatientsContext } from "@/context/patientsContext";
import VisitSkeleton from "./visitSkeleton";
import { AlertContext } from "@/context/alertContext";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/shadcn/accordion";
import { CalendarIcon } from "lucide-react";
import { save } from "@tauri-apps/plugin-dialog";
import VisitDialog from "./Visit/visitDialog";

function PatientInfo({ selected }) {
  const [visitas, setVisitas] = useState([]);
  const [dialogContent, setDialogContent] = useState("add");
  const { setPatients, setSelected } = useContext(PatientsContext);
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const { showToast, toast } = useContext(AlertContext);
  const [toEdit, setToEdit] = useState(null);

  // Get visits on mount
  useEffect(() => {
    invoke("get_visits_comm", {
      patientId: selected.id,
    })
      .then((r) => {
        setVisitas(
          r.sort((a, b) => new Date(b.datetime) - new Date(a.datetime))
        );
      })
      .catch((err) => {
        // No visits
        console.error(err);
      })
      .finally(() => {
        setIsLoading(false);
      });
  }, []);

  return (
    <Dialog open={isDialogOpen}>
      <DialogContent showCloseButton={false}>
        {dialogContent === "add" ? (
          <>
            <VisitDialog
              setIsDialogOpen={setIsDialogOpen}
              setVisitas={setVisitas}
              toEdit={toEdit}
              selected={selected}
            />
          </>
        ) : dialogContent === "edit" ? (
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
                try {
                  await invoke("update_patient_comm", {
                    patient: form,
                  });
                  setSelected(form);
                  setPatients((prev) =>
                    prev.map((p, i) => (form.id === p.id ? form : p))
                  );
                  setIsDialogOpen(false);
                  showToast("Cambios guardados", "", null, "success");
                } catch (error) {
                  if (error.includes("National")) {
                    showToast(
                      "Ocurrio un error al editar al paciente",
                      `Un paciente con el DNI ${form.national_id} ya se encuentra registrado`,
                      null,
                      "error"
                    );
                  }
                }
              }}
            />
          </>
        ) : (
          <>
            <DialogHeader>
              <DialogTitle>Exportar PDF</DialogTitle>
              <Separator className="my-2" />
              <div className="flex flex-col">
                <p>Esto va a crear un pdf con:</p>
                <ul className="list-disc ml-5">
                  <li>Datos del paciente</li>
                  <li>Historial completo de visitas</li>
                  <li>Todos los archivos adjuntos en visitas</li>
                </ul>

                <Separator className="my-5" />

                <div className="self-end">
                  <Button
                    onClick={() => {
                      setIsDialogOpen(false);
                    }}
                    variant="outline"
                    className="mr-2"
                  >
                    Cancelar
                  </Button>
                  <Button
                    onClick={async () => {
                      const path = await save({
                        filters: [
                          {
                            name: "Historia medica",
                            extensions: ["pdf"],
                          },
                        ],
                        defaultPath: `Historia-medica-${selected.name}-${selected.surname}.pdf`,
                      });
                      if (!path) return;
                      setIsDialogOpen(false);

                      const t = toast.loading("Exportando PDF");

                      const pdf = await invoke("export_to_pdf_comm", {
                        patientId: selected.id,
                        savePath: path,
                      });

                      toast.success("PDF exportado", {
                        description: pdf.path,
                        id: t,
                      });
                    }}
                  >
                    Exportar
                  </Button>
                </div>
              </div>
            </DialogHeader>
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

          <div>
            <Button
              variant="outline"
              className="mr-2"
              onClick={() => {
                setDialogContent("export");
                setIsDialogOpen(true);
              }}
            >
              Exportar a PDF
            </Button>

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
        </div>

        <Separator className="my-5" />
        {/* Visits */}
        <div className="flex items-center justify-between mb-5">
          <span className="flex items-center gap-2 text-xl">
            <ClipboardList className="text-text-primary" />
            Historial de Visitas ({visitas.length})
          </span>

          <Button
            onClick={() => {
              setDialogContent("add");
              setIsDialogOpen(true);
              setToEdit(null);
            }}
          >
            Agregar Visita
          </Button>
        </div>

        {/* scroll */}
        <div className="h-full grid min-h-0 relative rounded-md border">
          <div className="flex flex-col overflow-auto">
            {/* Show when there are no visits */}
            {!visitas.length && !isLoading ? (
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

            {isLoading ? (
              <VisitSkeleton />
            ) : (
              <Accordion type="single" collapsible>
                {visitas.map((x, i, arr) => (
                  <AccordionItem value={`item-${i + 1}`}>
                    <AccordionTrigger
                      className="group items-center px-5"
                      component={
                        <Button
                          variant="outline"
                          className="editarVisita group-data-[state=open]:opacity-100 transition-opacity opacity-0"
                          onClick={() => {
                            setDialogContent("add");
                            setIsDialogOpen(true);
                            setToEdit(x);
                          }}
                        >
                          Editar Visita
                        </Button>
                      }
                    >
                      <div className="flex items-center">
                        <CalendarIcon className="mr-5 text-blue-400" />

                        <div className="flex flex-col">
                          <span className="font-bold">
                            {new Intl.DateTimeFormat("es-ES", {
                              day: "numeric",
                              month: "long",
                              year: "numeric",
                            })
                              .format(new Date(x.datetime))
                              .replace(/ de (\d+)/, ", $1")}
                          </span>
                          <span className="text-text-secondary font-normal">
                            {x.reason}
                          </span>
                        </div>
                      </div>
                    </AccordionTrigger>

                    <AccordionContent className="bg-[#f9fafb]">
                      <Visita readOnly key={x.id} className="p-5" visita={x} />
                    </AccordionContent>
                  </AccordionItem>
                ))}
              </Accordion>
            )}
          </div>
        </div>
      </div>
    </Dialog>
  );
}

export default PatientInfo;
