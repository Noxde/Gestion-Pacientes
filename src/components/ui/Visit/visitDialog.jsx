import React, { useContext, useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/shadcn/dialog";
import { Separator } from "@/components/shadcn/separator";
import { Button } from "@/components/shadcn/button";
import Visita from "./visita";
import { AlertContext } from "@/context/alertContext";
import { invoke } from "@tauri-apps/api/core";

function VisitDialog({ setVisitas, setIsDialogOpen, toEdit, selected }) {
  const [toAdd, setToAdd] = useState({});
  const { showToast, toast } = useContext(AlertContext);

  console.log(toEdit);
  async function saveVisit() {
    const { datetime, reason, diagnosis, treatment, notes, docs } = toAdd;

    try {
      const res = await invoke("save_visit_comm", {
        newVisit: {
          id: 1,
          patient_id: selected.id,
          title: "Placeholder not used yet",
          docs,
          datetime: new Date(
            datetime.getTime() - datetime.getTimezoneOffset() * 60000 // Fix for UTC date
          )
            .toJSON()
            .replace("Z", ""),
          reason,
          diagnosis,
          treatment,
          notes,
        },
        patientId: selected.id,
      });
      setVisitas((prev) =>
        [res, ...prev].sort(
          (a, b) => new Date(b.datetime) - new Date(a.datetime)
        )
      );
      setToAdd({});
      setIsDialogOpen(false);
      showToast("Visita agregada", "", null, "success");
    } catch (err) {
      showToast("Ocurrio un error al agregar la visita", err, null, "error");
      console.error(err);
    }
  }

  async function editVisit() {
    const { datetime, reason, diagnosis, treatment, notes, docs } = toAdd;

    try {
      const res = await invoke("update_visit_comm", {
        visit: {
          id: 1,
          patient_id: selected.id,
          title: "Placeholder not used yet",
          docs,
          datetime,
          reason,
          diagnosis,
          treatment,
          notes,
        },
      });
      setVisitas((prev) => {
        const index = prev.findIndex((x) => x.id === toEdit.id);
        prev[index] = res;

        return [...prev].sort(
          (a, b) => new Date(b.datetime) - new Date(a.datetime)
        );
      });
      setToAdd({});
      setIsDialogOpen(false);
      showToast("Visita Editada", "", null, "success");
    } catch (err) {
      showToast("Ocurrio un error al editar la visita", err, null, "error");
      console.error(err);
    }
  }

  return (
    <>
      <DialogHeader>
        <DialogTitle>{toEdit ? "Editar" : "Agregar"} Visita</DialogTitle>
      </DialogHeader>

      <Separator />
      <Visita
        visita={toEdit}
        readOnly={false}
        onChange={(e) => {
          console.log(e);
          setToAdd(e);
        }}
      />
      <Separator />

      <div className="flex gap-2 justify-self-end">
        <Button onClick={() => setIsDialogOpen(false)} variant={"outline"}>
          Cancelar
        </Button>
        <Button onClick={toEdit ? editVisit : saveVisit}>Agregar</Button>
      </div>
    </>
  );
}

export default VisitDialog;
